use std::env;

use crate::{
    controller::{self, TiraResponse, TIRA_AUTH_COOKIE},
    models::{success::StandardResponse, Login, User},
    service, TiraDbConn,
};
use rocket::{
    http::{Cookie, CookieJar, Status},
    serde::json::Json,
};
use time::{Duration, OffsetDateTime};

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
    let remember_me = login_info.remember_me;

    let uuid_and_user = service::sessions::login(&conn, login_info).await?;

    let expiration = if remember_me {
        None
    } else {
        let mut expiration = OffsetDateTime::now_utc();
        expiration +=
            Duration::minutes(env::var("SESSION_LENGTH_MINUTES").unwrap().parse().unwrap());
        Some(expiration)
    };

    let cookie = Cookie::build(TIRA_AUTH_COOKIE, uuid_and_user.0)
        .expires(expiration)
        .finish();
    cookies.add(cookie);

    Ok(controller::create_success_response(
        Status::Created,
        uuid_and_user.1,
    ))
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
pub async fn logout_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
) -> TiraResponse<StandardResponse> {
    let (user_id, session_uuid) = controller::authentication(&conn, cookies).await?;

    service::sessions::logout(&conn, user_id, session_uuid).await?;
    cookies.remove(Cookie::named(TIRA_AUTH_COOKIE));

    let message = "Successfully logged out user!".to_string();
    let response = StandardResponse { message };
    Ok(controller::create_success_response_ok(response))
}
