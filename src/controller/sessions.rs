use rocket::{
    http::{Cookie, CookieJar, SameSite, Status},
    response::status::{self, Custom},
    serde::json::Json,
};

use crate::{controller::{self, TiraResponse, TIRA_AUTH_COOKIE, TiraMessage}, models::User};
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
) -> Result<Custom<Json<User>>, Custom<Json<TiraMessage>>> {
    let mut login_info = login_info.0;
    login_info.password = service::security::sha256(&login_info.password);

    let uuid_and_user =
        controller::standardize_error_response(service::sessions::login(&conn, login_info).await)?;

    cookies.add(Cookie::new(TIRA_AUTH_COOKIE, uuid_and_user.0));
    Ok(status::Custom(Status::Created, Json(uuid_and_user.1)))
}

/// OPTIONS endpoint for login.
#[options("/login")]
pub async fn login_options_endpoint() {}

/// Endpoint for logging out.
///
/// **POST /logout**
///
/// Requires authentication.
#[post("/logout")]
pub async fn logout_endpoint(conn: TiraDbConn, cookies: &CookieJar<'_>) -> TiraResponse<()> {
    let user_id = controller::authentication(&conn, cookies).await?;
    controller::standardize_error_response(service::sessions::logout(&conn, user_id).await)?;

    // cookies.remove(Cookie::named(TIRA_AUTH_COOKIE));
    Ok(status::Custom(Status::NoContent, Json(())))
}
