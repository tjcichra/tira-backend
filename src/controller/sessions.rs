use crate::{
    controller::{self, TiraResponse, TIRA_AUTH_COOKIE},
    models::{Login, User, success::StandardResponse},
    service,
    TiraDbConn
};
use rocket::{
    http::{Cookie, CookieJar, Status, private::cookie::Expiration},
    response::status,
    serde::json::Json,
};
use time::{OffsetDateTime, Duration};

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
) -> TiraResponse<User> {
    let mut login_info = login_info.0;
    login_info.password = service::security::sha256(&login_info.password);

    let uuid_and_user = service::sessions::login(&conn, login_info).await?;

    let mut expiration = OffsetDateTime::now_utc();
    expiration += Duration::hour();

    let cookie = Cookie::build(TIRA_AUTH_COOKIE, uuid_and_user.0).expires(expiration).finish();
    cookies.add(cookie);

    Ok(controller::create_success_response(Status::Created, uuid_and_user.1))
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
pub async fn logout_endpoint(conn: TiraDbConn, cookies: &CookieJar<'_>) -> TiraResponse<StandardResponse> {
    let user_id = controller::authentication(&conn, cookies).await?;
    service::sessions::logout(&conn, user_id).await?;
    cookies.remove(Cookie::named(TIRA_AUTH_COOKIE));

    let message = format!("Successfully logged out user!");
    let response = StandardResponse { message };
    Ok(controller::create_success_response_ok(response))
}
