use crate::{models::Session, TiraState};
use anyhow::Result;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use axum_extra::extract::CookieJar;
pub mod assignments;
pub mod categories;
pub mod comments;
pub mod images;
pub mod sessions;
pub mod tickets;
pub mod users;
use anyhow::anyhow;

const TIRA_AUTH_COOKIE: &str = "tirauth";

pub struct TiraError(anyhow::Error);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for TiraError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, TiraError>`. That way you don't need to do that manually.
impl<E> From<E> for TiraError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

// Service method that checks for authentication given a user's cookies
//
// Returns the user id of that user and their session uuid if they are authenticated or returns an error response
pub async fn authentication(
    State(state): State<TiraState>,
    cookie_jar: CookieJar,
    mut req: Request,
    next: Next,
) -> Result<Response, TiraError> {
    let cookie = cookie_jar.get(TIRA_AUTH_COOKIE);
    match cookie {
        Some(cookie) => {
            let session_uuid = cookie.value().to_string();

            let session = sqlx::query_as!(
                Session,
                "SELECT * FROM sessions WHERE uuid = $1 and expiration >= now()",
                session_uuid,
            )
            .fetch_one(&state.pool)
            .await?;

            req.extensions_mut().insert(session);

            Ok(next.run(req).await)
        }
        None => Err(TiraError(anyhow!("User not authenticated"))),
    }
}
