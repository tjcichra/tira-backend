use rocket::{http::{Cookie, CookieJar, Status}, serde::json::Json, response::status};

use crate::controller::{self, TIRA_AUTH_COOKIE, TiraResponse};
use crate::models::Login;
use crate::service;
use crate::TiraDbConn;

/// Endpoint for login.
///
/// **POST /login**
///
/// Example JSON Body:
///
/// {
///     "username": "testusername",
///     "password": "testsha256password"
/// }
#[post("/login", data = "<login_info>")]
pub async fn login_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    login_info: Json<Login>,
) -> TiraResponse<()> {
    let uuid = controller::standardize_error_response(service::sessions::login(&conn, login_info.0).await)?;

    cookies.add(Cookie::new(TIRA_AUTH_COOKIE, uuid));
    Ok(status::Custom(Status::Created, Json(())))
}