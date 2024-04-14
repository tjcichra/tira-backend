use std::env;

use super::TiraError;
use crate::{
    controller::TIRA_AUTH_COOKIE,
    models::{success::StandardResponse, Login, Session},
    service, TiraState,
};
use anyhow::Result;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use axum_extra::extract::CookieJar;
use cookie::{
    time::{Duration, OffsetDateTime},
    Cookie,
};
use log::info;

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
pub async fn login_endpoint(
    State(state): State<TiraState>,
    cookie_jar: CookieJar,
    login_info: Json<Login>,
) -> Result<Response, TiraError> {
    let mut login_info = login_info.0;
    login_info.password = service::security::sha256(&login_info.password);
    let remember_me = login_info.remember_me;

    let uuid_and_user = service::sessions::login(&state, login_info).await?;

    let expiration = if remember_me {
        None
    } else {
        let mut expiration = OffsetDateTime::now_utc();
        expiration +=
            Duration::minutes(env::var("SESSION_LENGTH_MINUTES").unwrap().parse().unwrap());
        Some(expiration)
    };

    let cookie = Cookie::build((TIRA_AUTH_COOKIE, uuid_and_user.0)).expires(expiration);

    Ok((cookie_jar.add(cookie), StatusCode::CREATED).into_response())
}

/// Endpoint for logging out.
///
/// **POST /logout**
///
/// Requires authentication.
pub async fn logout_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
    cookie_jar: CookieJar,
) -> Result<Response, TiraError> {
    service::sessions::logout(&state, session.user_id, session.uuid).await?;

    let message = "Successfully logged out user!".to_string();
    let response = StandardResponse { message };
    Ok((cookie_jar.remove(TIRA_AUTH_COOKIE), Json(response)).into_response())
}
