pub mod assignments;
pub mod categories;
pub mod comments;
pub mod images;
pub mod sessions;
pub mod tickets;
pub mod users;

use chrono::Utc;
use diesel::{result::Error as QueryError, ExpressionMethods, QueryDsl, RunQueryDsl};
use rocket::http::{CookieJar, Status};
use rocket::response::status::Custom;
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

#[derive(Debug, Serialize)]
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

pub fn convert(error: QueryError) -> TiraErrorResponse {
    match error {
        QueryError::NotFound => {
            create_error_response(Status::NotFound, "Resource not found".to_string())
        }
        x => create_error_response_500(x.to_string()),
    }
}

pub fn create_error_response(status: Status, message: String) -> TiraErrorResponse {
    Custom(status, Json(TiraMessage { message }))
}

pub fn create_error_response_500(message: String) -> TiraErrorResponse {
    create_error_response(Status::InternalServerError, message)
}

pub fn create_success_response<T>(status: Status, response_data: T) -> TiraSuccessResponse<T> {
    Custom(status, Json(response_data))
}

pub fn create_success_response_ok<T>(response_data: T) -> TiraSuccessResponse<T> {
    create_success_response(Status::Ok, response_data)
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

/// Service method that checks for authentication given a user's cookies
///
/// Returns the user id of that user and their session uuid if they are authenticated or returns an error response
async fn authentication(
    conn: &TiraDbConn,
    cookies: &CookieJar<'_>,
) -> Result<(i64, String), TiraErrorResponse> {
    use crate::schema::sessions::dsl::*;

    let cookie = cookies.get(TIRA_AUTH_COOKIE);

    match cookie {
        Some(cookie) => {
            let session_uuid = cookie.value().to_string();
            let session_uuid_clone = session_uuid.clone();

            let session = conn
                .run(|c| sessions.filter(uuid.eq(session_uuid)).first::<Session>(c))
                .await;

            match session {
                Ok(session) => {
                    match session.expiration {
                        Some(exp) => {
                            if Utc::now().naive_utc() > exp {
                                // Delete expired session
                                conn.run(|c| {
                                    diesel::delete(sessions.filter(uuid.eq(session_uuid_clone)))
                                        .execute(c)
                                })
                                .await
                                .unwrap();

                                // Return error message saying session has expired
                                Err(status::Custom(
                                    Status::Forbidden,
                                    Json(TiraMessage {
                                        message: "Session has expired, please log in again."
                                            .to_string(),
                                    }),
                                ))
                            } else {
                                Ok((session.user_id, session_uuid_clone))
                            }
                        }
                        None => Ok((session.user_id, session_uuid_clone)),
                    }
                }
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
