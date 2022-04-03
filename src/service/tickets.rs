use crate::{dao::tickets, TiraDbConn, models::{Ticket, create::{CreateComment, CreateTicket, CreateAssignmentWithUserId}, Assignment, Comment}, controller::TiraMessage, service};
use diesel::result::Error as QueryError;

/// Service function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    conn: &TiraDbConn,
    assignee_id: CreateAssignmentWithUserId,
    ticket_id: i64,
    assigner_id: i64
) -> Result<(), TiraMessage> {
    let assignments_created = tickets::create_assignment_by_ticket_id_and_assigner_id(conn, assignee_id, ticket_id, assigner_id).await;
    service::check_only_one_row_changed(assignments_created)
}

/// Service function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    conn: &TiraDbConn,
    comment: CreateComment,
    ticket_id: i64,
    commenter_id: i64,
) -> Result<(), TiraMessage> {
    let comments_created = tickets::create_comment_by_ticket_id_and_commenter_id(conn, comment, ticket_id, commenter_id).await;
    service::check_only_one_row_changed(comments_created)
}

/// Service function for creating a ticket by reporter id.
pub async fn create_ticket_by_reporter_id(conn: &TiraDbConn, ticket: CreateTicket, reporter_id: i64) -> Result<(), TiraMessage> {
    let tickets_created = tickets::create_ticket_by_reporter_id(conn, ticket, reporter_id).await;
    service::check_only_one_row_changed(tickets_created)
}

/// Service function for retrieving assignments by ticket id.
pub async fn get_assignments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id: i64,
) -> Result<Vec<Assignment>, QueryError> {
    tickets::get_assignments_by_ticket_id(conn, ticket_id).await
}

/// Service function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(conn: &TiraDbConn, ticket_id: i64) -> Result<Vec<Comment>, QueryError> {
    tickets::get_comments_by_ticket_id(conn, ticket_id).await
}

/// Service function for retrieving a ticket by id.
pub async fn get_ticket_by_id(conn: &TiraDbConn, ticket_id: i64) -> Result<Ticket, QueryError> {
    tickets::get_ticket_by_id(conn, ticket_id).await
}

/// Service function for retrieving all tickets.
pub async fn get_tickets(conn: &TiraDbConn) -> Result<Vec<Ticket>, QueryError> {
    tickets::get_tickets(conn).await
}
