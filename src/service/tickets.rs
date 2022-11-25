use crate::{
    controller::{self, TiraErrorResponse},
    dao::{self, tickets},
    models::{
        create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
        patch::UpdateTicket,
        Assignment, Comment, Ticket, TicketWithoutDescription,
    },
    service, TiraDbConn,
};
use regex::Regex;
use rocket::http::Status;

/// Service function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    conn: &TiraDbConn,
    assignee_id: CreateAssignmentWithUserId,
    ticket_id: i64,
    assigner_id: i64,
) -> Result<i64, TiraErrorResponse> {
    dao::tickets::create_assignment_by_ticket_id_and_assigner_id(
        conn,
        assignee_id,
        ticket_id,
        assigner_id,
    )
    .await
    .map_err(controller::convert)
}

/// Service function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    conn: &TiraDbConn,
    comment: CreateComment,
    ticket_id: i64,
    commenter_id: i64,
) -> Result<i64, TiraErrorResponse> {
    let regex = Regex::new(r"(</?[^>]+(>|$)|&nbsp;|\s)").unwrap();
    let content_without_tags: String = regex.replace_all(&comment.content, "").into();

    if content_without_tags.is_empty() {
        return Err(controller::create_error_response(
            Status::BadRequest,
            "Comment cannot be blank!".to_string(),
        ));
    }

    dao::tickets::create_comment_by_ticket_id_and_commenter_id(
        conn,
        comment,
        ticket_id,
        commenter_id,
    )
    .await
    .map_err(controller::convert)
}

/// Service function for creating a ticket by reporter id.
pub async fn create_ticket_by_reporter_id(
    conn: &TiraDbConn,
    ticket: CreateTicket,
    reporter_id: i64,
) -> Result<i64, TiraErrorResponse> {
    if ticket.subject.is_empty() {
        return Err(controller::create_error_response(
            Status::BadRequest,
            "Subject can not be empty".to_string(),
        ));
    }

    match ticket.status.as_str() {
        "Backlog" | "In Progress" | "Not Deployed Yet" | "Done" | "Closed" => (),
        _ => {
            return Err(controller::create_error_response(
                Status::BadRequest,
                "Status must be 'Backlog', 'In Progress', 'Not Deployed Yet', 'Done', or 'Closed'"
                    .to_string(),
            ));
        }
    }

    match ticket.priority.as_str() {
        "Low" | "Medium" | "High" => (),
        _ => {
            return Err(controller::create_error_response(
                Status::BadRequest,
                "Priority must be 'Low', 'Medium', or 'High'".to_string(),
            ));
        }
    }

    dao::tickets::create_ticket_by_reporter_id(conn, ticket, reporter_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving assignments by ticket id.
pub async fn get_assignments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id: i64,
) -> Result<Vec<Assignment>, TiraErrorResponse> {
    dao::tickets::get_assignments_by_ticket_id(conn, ticket_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id: i64,
) -> Result<Vec<Comment>, TiraErrorResponse> {
    dao::tickets::get_comments_by_ticket_id(conn, ticket_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving a ticket by id.
pub async fn get_ticket_by_id(
    conn: &TiraDbConn,
    ticket_id: i64,
) -> Result<Ticket, TiraErrorResponse> {
    dao::tickets::get_ticket_by_id(conn, ticket_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving tickets by ids.
pub async fn get_tickets_by_ids(
    conn: &TiraDbConn,
    ticket_ids: Vec<i64>,
) -> Result<Vec<Ticket>, TiraErrorResponse> {
    dao::tickets::get_tickets_by_ids(conn, ticket_ids)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving all tickets.
pub async fn get_tickets(
    conn: &TiraDbConn,
    limit: Option<i64>,
    offset: Option<i64>,
    filter_reporter_id: Option<i64>,
    filter_open: Option<bool>,
) -> Result<(Vec<TicketWithoutDescription>, i64), TiraErrorResponse> {
    let tickets = dao::tickets::get_tickets(conn, limit, offset, filter_reporter_id, filter_open)
        .await
        .map_err(controller::convert)?;

    let total_count = dao::tickets::get_total_ticket_count(conn)
        .await
        .map_err(controller::convert)?;

    Ok((tickets, total_count))
}

/// Service function for updating a ticket by id.
pub async fn update_ticket_by_id(
    conn: &TiraDbConn,
    ticket: UpdateTicket,
    ticket_id: i64,
) -> Result<(), TiraErrorResponse> {
    let tickets_updated = tickets::update_ticket_by_id(conn, ticket, ticket_id)
        .await
        .map_err(controller::convert)?;
    service::check_only_one_row_changed(tickets_updated)
}

/// Service function for updating assignments by ticket id.
pub async fn update_assignments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id: i64,
    assignee_ids: Vec<i64>,
    assigner_id: i64,
) -> Result<(), TiraErrorResponse> {
    dao::assignments::update_assignments_by_ticket_id(conn, ticket_id, assignee_ids, assigner_id)
        .await
        .map_err(controller::convert)?;
    Ok(())
}
