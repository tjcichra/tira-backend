pub mod categories;
pub mod sessions;
pub mod tickets;
pub mod users;

use chrono::Utc;
use diesel::{
    result::{Error as QueryError},
    ExpressionMethods, QueryDsl, RunQueryDsl,
};
use rocket::http::{CookieJar, Status};
use rocket::response::{content, status};
use rocket::{
    http::ContentType,
    serde::{json::Json, Serialize},
    Request,
};

use crate::models::Session;
use crate::TiraDbConn;

const TIRA_AUTH_COOKIE: &str = "tirauth";

pub type TiraSuccessResponse<T> = status::Custom<Json<T>>;
pub type TiraErrorResponse = status::Custom<Json<TiraMessage>>;

pub type TiraResponse<T> = Result<TiraSuccessResponse<T>, TiraErrorResponse>;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TiraMessage {
    pub message: String,
}

/// For converting diesel's query errors to generic tira errors.
impl From<QueryError> for TiraMessage {
    fn from(query_error: QueryError) -> Self {
        Self {
            message: query_error.to_string(),
        }
    }
}

#[catch(404)]
pub fn not_found(req: &Request) -> content::Custom<Json<TiraMessage>> {
    let custom = ContentType::new("application", "problem+json");

    content::Custom(
        custom,
        Json(TiraMessage {
            message: format!("Sorry, '{}' is not a valid path.", req.uri()),
        }),
    )
}

/// Method for
fn standardize_response<T, E: Into<TiraMessage>>(
    result: Result<T, E>,
    success_status: Status,
) -> TiraResponse<T> {
    result
        .map(|value| status::Custom(success_status, Json(value)))
        .map_err(|err| {
            let error = err.into();
            status::Custom(Status::InternalServerError, Json(error))
        })
}

fn standardize_response_ok<T, E: Into<TiraMessage>>(result: Result<T, E>) -> TiraResponse<T> {
    standardize_response(result, Status::Ok)
}

fn standardize_error_response<T, E: Into<TiraMessage>>(result: Result<T, E>) -> Result<T, TiraErrorResponse> {
    result.map_err(|err| status::Custom(Status::InternalServerError, Json(err.into())))
}

/// Service method that checks for authentication given a user's cookies
///
/// Returns the user id of that user if they are authenticated or returns an error response
async fn authentication(
    conn: &TiraDbConn,
    cookies: &CookieJar<'_>,
) -> Result<i64, TiraErrorResponse> {
    use crate::schema::sessions::dsl::*;

    let cookie = cookies.get(TIRA_AUTH_COOKIE);

    match cookie {
        Some(cookie) => {
            let session_uuid = cookie.value().to_string();
            let session_uuid_2 = session_uuid.clone();

            let session = conn
                .run(|c| sessions.filter(uuid.eq(session_uuid)).first::<Session>(c))
                .await;

            match session {
                Ok(session) => {
                    if Utc::now().naive_utc() > session.expiration {
                        // Delete expired session
                        conn.run(|c| {
                            diesel::delete(sessions.filter(uuid.eq(session_uuid_2))).execute(c)
                        }).await.unwrap();

                        // Return error message saying session has expired
                        Err(status::Custom(
                            Status::Forbidden,
                            Json(TiraMessage {
                                message: "Session has expired, please log in again.".to_string(),
                            }),
                        ))
                    } else {
                        Ok(session.user_id)
                    }
                },
                Err(error) => Err(status::Custom(Status::Forbidden, Json(error.into()))),
            }
        }
        None => Err(status::Custom(
            Status::Forbidden,
            Json(TiraMessage {
                message: "User not authenticated.".to_string(),
            }),
        )),
    }
}
