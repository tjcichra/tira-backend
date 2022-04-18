use crate::{
    controller::{self, TiraResponse, TIRA_AUTH_COOKIE},
    models::{Login, User},
    service,
    TiraDbConn
};
use rocket::{
    http::{Cookie, CookieJar, Status},
    response::status,
    serde::json::Json,
};

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

    cookies.add(Cookie::new(TIRA_AUTH_COOKIE, uuid_and_user.0));

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
pub async fn logout_endpoint(conn: TiraDbConn, cookies: &CookieJar<'_>) -> TiraResponse<()> {
    let user_id = controller::authentication(&conn, cookies).await?;
    service::sessions::logout(&conn, user_id).await?;

    // cookies.remove(Cookie::named(TIRA_AUTH_COOKIE));
    Ok(status::Custom(Status::NoContent, Json(())))
}
