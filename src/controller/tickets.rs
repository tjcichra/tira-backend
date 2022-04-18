use crate::controller::{self, TiraMessage, TiraResponse};
use crate::models::TicketWithoutDescription;
use crate::models::patch::UpdateTicket;
use crate::models::success::{AlteredResourceResponse, EditTicketResponse};
use crate::models::{
    create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
    Assignment, Comment, Ticket,
};
use crate::service::{self, tickets};
use crate::TiraDbConn;
use rocket::http::{CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;

/// Endpoint for creating an assignment for a ticket.
///
/// Requires authentication.
///
/// **POST /tickets/<ticket_id>/assignments**
///
/// Example JSON Body:
///
/// {
///     "user_id": "123"
/// }
#[post("/tickets/<ticket_id>/assignments", data = "<create_assignment_json>")]
pub async fn create_assignment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_assignment_json: Json<CreateAssignmentWithUserId>,
    ticket_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    let user_id = controller::authentication(&conn, cookies).await?;
    let created_assignment_id = tickets::create_assignment_by_ticket_id_and_assigner_id(
        &conn,
        create_assignment_json.0,
        ticket_id,
        user_id,
    )
    .await?;

    let message = format!("Successfully created assignment!");
    let response = AlteredResourceResponse { message, id: created_assignment_id };
    Ok(controller::create_success_response(Status::Created, response))
}

/// Endpoint for creating a comment for a ticket.
///
/// Requires authentication.
///
/// **POST /tickets/<ticket_id>/comments**
///
/// Example JSON Body:
///
/// {
///     "content": "This is a comment"
/// }
#[post("/tickets/<ticket_id>/comments", data = "<create_comment_json>")]
pub async fn create_comment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_comment_json: Json<CreateComment>,
    ticket_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    let user_id = controller::authentication(&conn, cookies).await?;
    let created_comment_id = tickets::create_comment_by_ticket_id_and_commenter_id(
        &conn,
        create_comment_json.0,
        ticket_id,
        user_id,
    )
    .await?;

    let message = format!("Successfully created comment!");
    let response = AlteredResourceResponse { message, id: created_comment_id };
    Ok(controller::create_success_response(Status::Created, response))
}

/// Endpoint for creating a ticket.
///
/// Requires authentication.
///
/// **POST /tickets**
///
/// Example JSON Body:
///
/// {
///     "category_id": "123",
///     "subject": "Finish Tira",
///     "description": "Finish working on the code for Tira",
///     "status": "IN PROGRESS",
///     "priority": "3"
/// }
#[post("/tickets", data = "<create_ticket_json>")]
pub async fn create_ticket_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_ticket_json: Json<CreateTicket>,
) -> Result<Custom<Json<AlteredResourceResponse>>, Custom<Json<TiraMessage>>> {
    let user_id = controller::authentication(&conn, cookies).await?;
    let created_ticket_id = service::tickets::create_ticket_by_reporter_id(&conn, create_ticket_json.0, user_id)
        .await?;

    let message = format!("Successfully created ticket!");
    let response = AlteredResourceResponse { message, id: created_ticket_id };
    Ok(controller::create_success_response(Status::Created, response))
}

/// Endpoint for retrieving all assignments for a ticket.
///
/// **GET /tickets/<ticket_id>/assignments**
#[get("/tickets/<ticket_id>/assignments")]
pub async fn get_assignments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> TiraResponse<Vec<Assignment>> {
    let assignments = service::tickets::get_assignments_by_ticket_id(&conn, ticket_id).await?;
    Ok(controller::create_success_response_ok(assignments))
}

/// Endpoint for retrieving all comments for a ticket.
///
/// **GET /tickets/<ticket_id>/comments**
#[get("/tickets/<ticket_id>/comments")]
pub async fn get_comments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> TiraResponse<Vec<Comment>> {
    let comments = service::tickets::get_comments_by_ticket_id(&conn, ticket_id).await?;
    Ok(controller::create_success_response_ok(comments))
}

/// Endpoint for retrieving a ticket.
///
/// **GET /tickets/<ticket_id>**
#[get("/tickets/<ticket_id>")]
pub async fn get_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i64) -> TiraResponse<Ticket> {
    let ticket = tickets::get_ticket_by_id(&conn, ticket_id).await?;
    Ok(controller::create_success_response_ok(ticket))
}

/// Endpoint for retrieving every ticket.
///
/// **GET /tickets**
///
/// Query Parameters:
///
/// archived: Used to filter tickets that were reported by a certain user. Takes a number value. (optional)
#[get("/tickets?<reporter>")]
pub async fn get_tickets_endpoint(
    conn: TiraDbConn,
    reporter: Option<i64>,
) -> TiraResponse<Vec<TicketWithoutDescription>> {
    let tickets = tickets::get_tickets(&conn, reporter).await?;
    Ok(controller::create_success_response_ok(tickets))
}

/// Endpoint for upating a ticket.
///
/// **PATCH /tickets/<ticket_id>**
#[patch("/tickets/<ticket_id>", data = "<update_ticket_json>")]
pub async fn patch_ticket_by_id_endpoint(conn: TiraDbConn, update_ticket_json: Json<UpdateTicket>, ticket_id: i64) -> TiraResponse<AlteredResourceResponse> {
    service::tickets::update_ticket_by_id(&conn, update_ticket_json.0, ticket_id).await?;

    let message = format!("Successfully edited ticket!");
    let response = AlteredResourceResponse { message, id: ticket_id };
    Ok(controller::create_success_response(Status::Created, response))
}
