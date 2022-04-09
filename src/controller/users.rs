use crate::controller::TiraResponse;
use crate::models::Assignment;
use crate::models::{create::CreateUser, User};
use crate::service::{users, self};
use crate::TiraDbConn;
use rocket::http::CookieJar;
use rocket::serde::json::Json;

use crate::controller;

/// Endpoint for archiving a specific user.
//
/// Requires authentication.
///
/// **DELETE /users/<user_id>**
#[delete("/users/<user_id>")]
pub async fn archive_user_by_id_endpoint(conn: TiraDbConn, cookies: &CookieJar<'_>, user_id: i64) -> TiraResponse<()> {
    controller::authentication(&conn, cookies).await?;
    controller::standardize_response_ok(users::archive_user_by_id(&conn, user_id).await)
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
) -> TiraResponse<()> {
    let mut user_json = user_json.0;
    user_json.password = service::security::sha256(&user_json.password);

    controller::standardize_response_ok(users::create_user(&conn, user_json).await)
}

/// Endpoint for retrieving all assignments for a user.
///
/// **GET /users/<user_id>/assignments**
#[get("/users/<user_id>/assignments")]
pub async fn get_assignments_by_user_id_endpoint(
    conn: TiraDbConn,
    user_id: i64
) -> TiraResponse<Vec<Assignment>> {
    let assignments = service::users::get_assignments_by_user_id(&conn, user_id).await;
    controller::standardize_response_ok(assignments)
}

/// Endpoint for retrieving a user.
///
/// **GET /users/<user_id>**
#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i64) -> TiraResponse<User> {
    controller::standardize_response_ok(users::get_user_by_id(&conn, user_id).await)
}

/// Endpoint for retrieving every user.
///
/// **GET /users**
/// 
/// Query Parameters:
/// 
/// archived: Used to filter users that are archived or not. Takes a boolean value. (optional)
#[get("/users?<archived>")]
pub async fn get_users_endpoint(conn: TiraDbConn, archived: Option<bool>) -> TiraResponse<Vec<User>> {
    controller::standardize_response_ok(users::get_users(&conn, archived).await)
}
