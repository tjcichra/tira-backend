use crate::controller::{self, TiraMessage, TiraResponse};
use crate::models::TicketWithoutDescription;
use crate::models::patch::UpdateTicket;
use crate::models::success::{AlteredResourceResponse, EditTicketResponse, TicketResponse, TicketWithoutDescriptionResponse, CommentResponse};
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
    let assignee_id = create_assignment_json.assignee_id;
    let created_assignment_id = tickets::create_assignment_by_ticket_id_and_assigner_id(
        &conn,
        create_assignment_json.0,
        ticket_id,
        user_id,
    )
    .await?;

    let assignee = service::users::get_user_by_id(&conn, assignee_id).await?;
    
    if let Some(email_address) = assignee.email_address {
        let assigner = service::users::get_user_by_id(&conn, user_id).await?;
        let ticket = service::tickets::get_ticket_by_id(&conn, ticket_id).await?;

        let body = service::emails::create_assignment_email_text(assigner, ticket);

        service::emails::send_email(&email_address, body);
    }

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
) -> TiraResponse<Vec<CommentResponse>> {
    let comments = service::tickets::get_comments_by_ticket_id(&conn, ticket_id).await?;
    let mut comments_response = Vec::new();

    for comment in comments {
        let commenter =  service::users::get_user_by_id(&conn, comment.commenter_id).await?;

        let comment_response = CommentResponse {
            id: comment.id,
            commenter,
            content: comment.content,
            commented: comment.commented,
        };

        comments_response.push(comment_response);
    }
    
    Ok(controller::create_success_response_ok(comments_response))
}

/// Endpoint for retrieving a ticket.
///
/// **GET /tickets/<ticket_id>**
#[get("/tickets/<ticket_id>")]
pub async fn get_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i64) -> TiraResponse<TicketResponse> {
    let ticket = service::tickets::get_ticket_by_id(&conn, ticket_id).await?;
    let reporter = service::users::get_user_by_id(&conn, ticket.reporter_id).await?;

    let ticket_response = TicketResponse {
        id: ticket.id,
        subject: ticket.subject,
        description: ticket.description,
        category_id: ticket.category_id,
        priority: ticket.priority,
        status: ticket.status,
        created: ticket.created,
        reporter
    };

    Ok(controller::create_success_response_ok(ticket_response))
}

/// Endpoint for retrieving every ticket.
///
/// **GET /tickets**
///
/// Query Parameters:
///
/// reporter: Used to filter tickets that were reported by a certain user. Takes a number value. (optional)
/// open: Used to filter tickets that are open or not. Takes a boolean. (optional)
#[get("/tickets?<reporter>&<open>")]
pub async fn get_tickets_endpoint(
    conn: TiraDbConn,
    reporter: Option<i64>,
    open: Option<bool>,
) -> TiraResponse<Vec<TicketWithoutDescriptionResponse>> {
    let tickets = tickets::get_tickets(&conn, reporter, open).await?;
    let mut tickets_response = Vec::new();

    for ticket in tickets {
        let reporter = service::users::get_user_by_id(&conn, ticket.reporter_id).await?;

        let ticket_response = TicketWithoutDescriptionResponse {
            id: ticket.id,
            subject: ticket.subject.clone(),
            description: ticket.description.clone(),
            category_id: ticket.category_id,
            priority: ticket.priority.clone(),
            status: ticket.status.clone(),
            created: ticket.created,
            reporter
        };

        tickets_response.push(ticket_response);
    }

    Ok(controller::create_success_response_ok(tickets_response))
}

/// Endpoint for updating a ticket.
///
/// **PATCH /tickets/<ticket_id>**
#[patch("/tickets/<ticket_id>", data = "<update_ticket_json>")]
pub async fn patch_ticket_by_id_endpoint(conn: TiraDbConn, update_ticket_json: Json<UpdateTicket>, ticket_id: i64) -> TiraResponse<AlteredResourceResponse> {
    service::tickets::update_ticket_by_id(&conn, update_ticket_json.0, ticket_id).await?;

    let message = format!("Successfully edited ticket!");
    let response = AlteredResourceResponse { message, id: ticket_id };
    Ok(controller::create_success_response_ok(response))
}
