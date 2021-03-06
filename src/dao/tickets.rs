use crate::{
    models::{
        create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
        patch::UpdateTicket,
        Assignment, Comment, Ticket, TicketWithoutDescription,
    },
    TiraDbConn,
};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    conn: &TiraDbConn,
    assignee_id_parameter: CreateAssignmentWithUserId,
    ticket_id_parameter: i64,
    assigner_id_parameter: i64,
) -> QueryResult<i64> {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(assignments)
            .values((
                assignee_id.eq(assignee_id_parameter.assignee_id),
                ticket_id.eq(ticket_id_parameter),
                assigner_id.eq(assigner_id_parameter),
                assigned.eq(Utc::now().naive_utc()),
            ))
            .returning(id)
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
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(comments)
            .values((
                &comment,
                ticket_id.eq(ticket_id_parameter),
                commenter_id.eq(commenter_id_parameter),
                commented.eq(Utc::now().naive_utc()),
            ))
            .returning(id)
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
            use crate::schema::tickets::dsl::*;

            diesel::insert_into(tickets)
                .values((
                    category_id.eq(ticket.category_id),
                    subject.eq(ticket.subject),
                    description.eq(ticket.description),
                    status.eq(ticket.status),
                    priority.eq(ticket.priority),
                    created.eq(Utc::now().naive_utc()),
                    reporter_id.eq(reporter_id_parameter),
                ))
                .returning(id)
                .get_result(c)?
        };

        {
            use crate::schema::assignments::dsl::*;

            let assignments_parameter: Vec<_> = ticket
                .assignee_ids
                .iter()
                .map(|assignee_id_parameter| {
                    (
                        ticket_id.eq(ticket_id_parameter),
                        assignee_id.eq(assignee_id_parameter),
                        assigner_id.eq(reporter_id_parameter),
                        assigned.eq(Utc::now().naive_utc()),
                    )
                })
                .collect();

            diesel::insert_into(assignments)
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
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        assignments
            .filter(ticket_id.eq(ticket_id_parameter))
            .load::<Assignment>(c)
    })
    .await
}

/// DAO function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id_parameter: i64,
) -> QueryResult<Vec<Comment>> {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        comments
            .filter(ticket_id.eq(ticket_id_parameter))
            .load::<Comment>(c)
    })
    .await
}

/// DAO function for retrieving a ticket by id.
pub async fn get_ticket_by_id(conn: &TiraDbConn, ticket_id: i64) -> QueryResult<Ticket> {
    use crate::schema::tickets::dsl::{id, tickets};

    conn.run(move |c| tickets.filter(id.eq(ticket_id)).first::<Ticket>(c))
        .await
}

/// DAO function for retrieving all tickets.
pub async fn get_tickets(
    conn: &TiraDbConn,
    filter_reporter_id: Option<i64>,
    filter_open: Option<bool>,
) -> QueryResult<Vec<TicketWithoutDescription>> {
    use crate::schema::tickets;

    conn.run(move |c| {
        let mut query = tickets::table.into_boxed();

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

        query.load(c)
    })
    .await
}

/// DAO function for updating a ticket by id.
pub async fn update_ticket_by_id(
    conn: &TiraDbConn,
    ticket: UpdateTicket,
    ticket_id: i64,
) -> QueryResult<usize> {
    conn.run(move |c| {
        use crate::schema::tickets::dsl;

        let subject = ticket.subject.map(|subject| dsl::subject.eq(subject));
        let priority = ticket.priority.map(|priority| dsl::priority.eq(priority));
        let status = ticket.status.map(|status| dsl::status.eq(status));

        diesel::update(dsl::tickets.filter(dsl::id.eq(ticket_id)))
            .set((
                subject,
                dsl::description.eq(ticket.description),
                dsl::category_id.eq(ticket.category_id),
                priority,
                status,
            ))
            .execute(c)
    })
    .await
}
