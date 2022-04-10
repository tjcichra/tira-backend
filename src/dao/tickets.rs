use chrono::Utc;
use crate::{
    models::{create::{CreateTicket, CreateComment, CreateAssignmentWithUserId}, Assignment, Comment, Ticket},
    TiraDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    conn: &TiraDbConn,
    assignee_id_parameter: CreateAssignmentWithUserId,
    ticket_id_parameter: i64,
    assigner_id_parameter: i64
) -> QueryResult<usize> {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(assignments)
            .values((
                assignee_id.eq(assignee_id_parameter.assignee_id),
                ticket_id.eq(ticket_id_parameter),
                assigner_id.eq(assigner_id_parameter),
                assigned.eq(Utc::now().naive_utc()),
            ))
            .execute(c)
    })
    .await
}

/// DAO function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    conn: &TiraDbConn,
    comment: CreateComment,
    ticket_id_parameter: i64,
    commenter_id_parameter: i64,
) -> QueryResult<usize> {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(comments)
            .values((
                &comment,
                ticket_id.eq(ticket_id_parameter),
                commenter_id.eq(commenter_id_parameter),
                commented.eq(Utc::now().naive_utc()),
            ))
            .execute(c)
    })
    .await
}

/// DAO function for creating a ticket by reporter id.
pub async fn create_ticket_by_reporter_id(conn: &TiraDbConn, ticket: CreateTicket, reporter_id_parameter: i64) -> QueryResult<usize> {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(tickets)
            .values((
                &ticket,
                created.eq(Utc::now().naive_utc()),
                reporter_id.eq(reporter_id_parameter),
            ))
            .execute(c)
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
pub async fn get_comments_by_ticket_id(conn: &TiraDbConn, ticket_id_parameter: i64) -> QueryResult<Vec<Comment>> {
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
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        tickets
            .filter(id.eq(ticket_id))
            .first::<Ticket>(c)
    })
    .await
}

/// DAO function for retrieving all tickets.
pub async fn get_tickets(conn: &TiraDbConn, filter_reporter_id: Option<i64>) -> QueryResult<Vec<Ticket>> {
    use crate::schema::tickets::dsl::*;

    match filter_reporter_id {
        Some(filter_reporter_id) => conn.run(move |c| tickets.filter(reporter_id.eq(filter_reporter_id)).load(c)).await,
        None => conn.run(|c| tickets.load(c)).await,
    }
}
