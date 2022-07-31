use crate::controller::TiraResponse;
use crate::models::patch::UpdateUser;
use crate::models::success::{AlteredResourceResponse, AssignmentResponse};
use crate::models::TicketWithReporterAsUser;
use crate::models::{create::CreateUser, User};
use crate::service;
use crate::TiraDbConn;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;

use crate::controller;

/// Endpoint for archiving a specific user.
//
/// Requires authentication.
///
/// **DELETE /users/<user_id>**
#[delete("/users/<user_id>")]
pub async fn archive_user_by_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    user_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    controller::authentication(&conn, cookies).await?;
    service::users::archive_user_by_id(&conn, user_id).await?;

    let message = "Successfully archived user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: user_id,
    };
    Ok(controller::create_success_response_ok(response))
}

/// Endpoint for creating a user.
///
/// **POST /users**
///
/// Example JSON Body:
///
/// {
///     "username": "testusername",
///     "password": "testsha256password",
///     "email_address": "testemailaddress",
///     "first_name": "testfirstname",
///     "last_name": "testtestname",
/// }
#[post("/users", data = "<user_json>")]
pub async fn create_user_endpoint(
    conn: TiraDbConn,
    user_json: Json<CreateUser>,
) -> TiraResponse<AlteredResourceResponse> {
    let mut user_json = user_json.0;
    user_json.password = service::security::sha256(&user_json.password);

    let created_user_id = service::users::create_user(&conn, user_json).await?;

    let message = "Successfully created user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: created_user_id,
    };
    Ok(controller::create_success_response(
        Status::Created,
        response,
    ))
}

/// Endpoint for retrieving all assignments for a user.
///
/// **GET /users/<user_id>/assignments**
#[get("/users/<user_id>/assignments")]
pub async fn get_assignments_by_user_id_endpoint(
    conn: TiraDbConn,
    user_id: i64,
) -> TiraResponse<Vec<AssignmentResponse>> {
    let assignments = service::users::get_assignments_by_user_id(&conn, user_id).await?;

    let mut ticket_ids = Vec::new();
    let mut assigner_ids = Vec::new();

    for assignment in &assignments {
        ticket_ids.push(assignment.ticket_id);
        assigner_ids.push(assignment.assigner_id);
    }

    let tickets = service::tickets::get_tickets_by_ids(&conn, ticket_ids).await?;
    let assigners = service::users::get_users_by_ids(&conn, assigner_ids).await?;

    let mut ticket_reporter_ids = Vec::new();

    for ticket in &tickets {
        ticket_reporter_ids.push(ticket.reporter_id);
    }

    let ticket_reporters = service::users::get_users_by_ids(&conn, ticket_reporter_ids).await?;

    let mut assignment_responses = Vec::new();
    for (index, assignment) in assignments.iter().enumerate() {
        let ticket = &tickets[index];
        let ticket_with_reporter_as_user = TicketWithReporterAsUser {
            id: ticket.id,
            subject: ticket.subject.clone(),
            description: ticket.description.clone(),
            category_id: ticket.category_id,
            priority: ticket.priority.clone(),
            status: ticket.status.clone(),
            created: ticket.created,
            reporter: ticket_reporters[index].clone(),
        };

        assignment_responses.push(AssignmentResponse {
            id: assignment.id,
            ticket: ticket_with_reporter_as_user,
            assigner: assigners[index].clone(),
            assigned: assignment.assigned,
        });
    }

    Ok(controller::create_success_response_ok(assignment_responses))
}

/// Endpoint for retrieving the current user.
///
/// Requires authentication.
///
/// **GET /users/current**
#[get("/users/current")]
pub async fn get_current_user_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
) -> TiraResponse<User> {
    let (user_id, _session_uuid) = controller::authentication(&conn, cookies).await?;

    let assignments = service::users::get_user_by_id(&conn, user_id).await?;
    Ok(controller::create_success_response_ok(assignments))
}

/// Endpoint for retrieving a user.
///
/// **GET /users/<user_id>**
#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i64) -> TiraResponse<User> {
    let tickets = service::users::get_user_by_id(&conn, user_id).await?;
    Ok(controller::create_success_response_ok(tickets))
}

/// Endpoint for retrieving every user.
///
/// **GET /users**
///
/// Query Parameters:
///
/// archived: Used to filter users that are archived or not. Takes a boolean value. (optional)
#[get("/users?<archived>")]
pub async fn get_users_endpoint(
    conn: TiraDbConn,
    archived: Option<bool>,
) -> TiraResponse<Vec<User>> {
    let users = service::users::get_users(&conn, archived).await?;
    Ok(controller::create_success_response_ok(users))
}

/// Endpoint for updating a user.
///
/// **PATCH /users/<user_id>**
#[patch("/users/<user_id>", data = "<update_user_json>")]
pub async fn patch_user_by_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    update_user_json: Json<UpdateUser>,
    user_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    let update_user = update_user_json.0;
    let (current_user_id, _session_uuid) = controller::authentication(&conn, cookies).await?;

    if user_id != current_user_id {
        return Err(controller::create_error_response(
            Status::BadRequest,
            "Cannot edit another person's user!".into(),
        ));
    }

    service::users::update_user_by_id(&conn, update_user, user_id).await?;

    let message = "Successfully edited user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: user_id,
    };
    Ok(controller::create_success_response_ok(response))
}
