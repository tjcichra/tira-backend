use crate::controller::TiraResponse;
use crate::models::patch::UpdateUser;
use crate::models::success::AlteredResourceResponse;
use crate::models::{create::CreateUser, User};
use crate::models::{Assignment, TicketWithoutDescription};
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

    let message = format!("Successfully archived user!");
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

    let message = format!("Successfully created user!");
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
) -> TiraResponse<Vec<Assignment>> {
    let assignments = service::users::get_assignments_by_user_id(&conn, user_id).await?;
    Ok(controller::create_success_response_ok(assignments))
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
    let user_id = controller::authentication(&conn, cookies).await?;

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
    let current_user_id = controller::authentication(&conn, cookies).await?;

    if user_id != current_user_id {
        return Err(controller::create_error_response(
            Status::BadRequest,
            "Cannot edit another person's user!".into(),
        ));
    }

    service::users::update_user_by_id(&conn, update_user, user_id).await?;

    let message = format!("Successfully edited user!");
    let response = AlteredResourceResponse {
        message,
        id: user_id,
    };
    Ok(controller::create_success_response_ok(response))
}
