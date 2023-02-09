use crate::{
    models::{
        create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
        patch::UpdateTicket,
        Assignment, Comment, Ticket, TicketWithoutDescription,
    },
    schema::tickets::BoxedQuery,
    TiraDbConn,
};
use chrono::Utc;
use diesel::{
    pg::Pg, query_builder::QueryFragment, AppearsOnTable, ExpressionMethods, QueryDsl, QueryResult,
    RunQueryDsl,
};

/// DAO function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    conn: &TiraDbConn,
    assignee_id_parameter: CreateAssignmentWithUserId,
    ticket_id_parameter: i64,
    assigner_id_parameter: i64,
) -> QueryResult<i64> {
    use crate::schema::assignments;

    conn.run(move |c| {
        diesel::insert_into(assignments::table)
            .values((
                assignments::assignee_id.eq(assignee_id_parameter.assignee_id),
                assignments::ticket_id.eq(ticket_id_parameter),
                assignments::assigner_id.eq(assigner_id_parameter),
                assignments::assigned.eq(Utc::now().naive_utc()),
            ))
            .returning(assignments::id)
            .get_result(c)
    })
    .await
}

/// DAO function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    conn: &TiraDbConn,
    comment: CreateComment,
    ticket_id_parameter: i64,
    commenter_id_parameter: i64,
) -> QueryResult<i64> {
    use crate::schema::comments;

    conn.run(move |c| {
        diesel::insert_into(comments::table)
            .values((
                &comment,
                comments::ticket_id.eq(ticket_id_parameter),
                comments::commenter_id.eq(commenter_id_parameter),
                comments::commented.eq(Utc::now().naive_utc()),
            ))
            .returning(comments::id)
            .get_result(c)
    })
    .await
}

/// DAO function for creating a ticket by reporter id and assigning those tickets.
///
/// Returns the id of the new ticket.
pub async fn create_ticket_by_reporter_id(
    conn: &TiraDbConn,
    ticket: CreateTicket,
    reporter_id_parameter: i64,
) -> QueryResult<i64> {
    conn.run(move |c| {
        let ticket_id_parameter = {
            use crate::schema::tickets;

            diesel::insert_into(tickets::table)
                .values((
                    tickets::category_id.eq(ticket.category_id),
                    tickets::subject.eq(ticket.subject),
                    tickets::description.eq(ticket.description),
                    tickets::status.eq(ticket.status),
                    tickets::priority.eq(ticket.priority),
                    tickets::created.eq(Utc::now().naive_utc()),
                    tickets::reporter_id.eq(reporter_id_parameter),
                ))
                .returning(tickets::id)
                .get_result(c)?
        };

        {
            use crate::schema::assignments;

            let assignments_parameter: Vec<_> = ticket
                .assignee_ids
                .iter()
                .map(|assignee_id_parameter| {
                    (
                        assignments::ticket_id.eq(ticket_id_parameter),
                        assignments::assignee_id.eq(assignee_id_parameter),
                        assignments::assigner_id.eq(reporter_id_parameter),
                        assignments::assigned.eq(Utc::now().naive_utc()),
                    )
                })
                .collect();

            diesel::insert_into(assignments::table)
                .values(&assignments_parameter)
                .execute(c)?;
        }

        Ok(ticket_id_parameter)
    })
    .await
}

/// DAO function for retrieving assignments by ticket id.
pub async fn get_assignments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id_parameter: i64,
) -> QueryResult<Vec<Assignment>> {
    use crate::schema::assignments;

    conn.run(move |c| {
        assignments::table
            .filter(assignments::ticket_id.eq(ticket_id_parameter))
            .load::<Assignment>(c)
    })
    .await
}

/// DAO function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id_parameter: i64,
) -> QueryResult<Vec<Comment>> {
    use crate::schema::comments;

    conn.run(move |c| {
        comments::table
            .filter(comments::ticket_id.eq(ticket_id_parameter))
            .load::<Comment>(c)
    })
    .await
}

/// DAO function for retrieving a ticket by id.
pub async fn get_ticket_by_id(conn: &TiraDbConn, ticket_id: i64) -> QueryResult<Ticket> {
    use crate::schema::tickets;

    conn.run(move |c| {
        tickets::table
            .filter(tickets::id.eq(ticket_id))
            .first::<Ticket>(c)
    })
    .await
}

/// DAO function for retrieving tickets by ids.
pub async fn get_tickets_by_ids(
    conn: &TiraDbConn,
    ticket_ids: Vec<i64>,
) -> QueryResult<Vec<Ticket>> {
    use crate::schema::tickets;

    if ticket_ids.is_empty() {
        return Ok(vec![]);
    }

    conn.run(move |c| {
        let mut query = tickets::table.into_boxed();

        for ticket_id in ticket_ids {
            query = query.or_filter(tickets::id.eq(ticket_id));
        }

        query.load(c)
    })
    .await
}

/// DAO function for retrieving all tickets.
pub async fn get_tickets(
    conn: &TiraDbConn,
    limit: Option<i64>,
    offset: Option<i64>,
    filter_reporter_id: Option<i64>,
    filter_open: Option<bool>,
    sort_by: Option<String>,
    order_by: Option<String>,
) -> QueryResult<Vec<TicketWithoutDescription>> {
    use crate::schema::tickets;

    conn.run(move |c| {
        let mut query = tickets::table.into_boxed();

        if let Some(limit) = limit {
            query = query.limit(limit);
        }

        if let Some(offset) = offset {
            query = query.offset(offset);
        }

        if let Some(filter_reporter_id) = filter_reporter_id {
            query = query.filter(tickets::reporter_id.eq(filter_reporter_id));
        }

        if let Some(filter_open) = filter_open {
            if filter_open {
                query = query
                    .filter(tickets::status.ne("Done"))
                    .filter(tickets::status.ne("Closed"));
            } else {
                query = query
                    .filter(tickets::status.eq("Done"))
                    .or_filter(tickets::status.eq("Closed"));
            }
        }

        if let Some(sort_by) = sort_by {
            let order_by = order_by.unwrap_or_else(|| "asc".to_string());
            query = match sort_by.as_str() {
                "id" => sort_by_column(query, tickets::id, order_by),
                "subject" => sort_by_column(query, tickets::subject, order_by),
                "priority" => sort_by_column(query, tickets::priority, order_by),
                "status" => sort_by_column(query, tickets::status, order_by),
                "created" => sort_by_column(query, tickets::created, order_by),
                _ => query,
            };
        }

        query.load(c)
    })
    .await
}

/// DAO function for retrieving the total ticket count.
pub async fn get_total_ticket_count(conn: &TiraDbConn) -> QueryResult<i64> {
    use crate::schema::tickets;

    conn.run(|c| tickets::table.count().get_result(c)).await
}

/// DAO function for updating a ticket by id.
pub async fn update_ticket_by_id(
    conn: &TiraDbConn,
    ticket: UpdateTicket,
    ticket_id: i64,
) -> QueryResult<usize> {
    conn.run(move |c| {
        use crate::schema::tickets;

        let subject = ticket.subject.map(|subject| tickets::subject.eq(subject));
        let priority = ticket
            .priority
            .map(|priority| tickets::priority.eq(priority));
        let status = ticket.status.map(|status| tickets::status.eq(status));

        diesel::update(tickets::dsl::tickets.filter(tickets::id.eq(ticket_id)))
            .set((
                subject,
                tickets::description.eq(ticket.description),
                tickets::category_id.eq(ticket.category_id),
                priority,
                status,
            ))
            .execute(c)
    })
    .await
}

fn sort_by_column<U: 'static>(
    query: BoxedQuery<'static, Pg>,
    column: U,
    order_by: String,
) -> BoxedQuery<'static, Pg>
where
    U: ExpressionMethods + QueryFragment<Pg> + AppearsOnTable<crate::schema::tickets::table>,
{
    match order_by.as_str() {
        "asc" => query.order_by(column.asc()),
        "desc" => query.order_by(column.desc()),
        _ => query.order_by(column.asc()),
    }
}
