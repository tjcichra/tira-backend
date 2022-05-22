use crate::controller;
use crate::controller::TiraResponse;
use crate::models::patch::UpdateComment;
use crate::models::success::AlteredResourceResponse;
use crate::service;
use crate::TiraDbConn;
use rocket::{http::CookieJar, serde::json::Json};

/// Endpoint for updating a comment.
///
/// Requires authentication.
///
/// **PATCH /comments/<comment_id>**
///
/// Example JSON Body:
///
/// {
///     "content": "This is a comment"
/// }
#[patch("/comments/<comment_id>", data = "<update_comment_json>")]
pub async fn patch_comment_by_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    update_comment_json: Json<UpdateComment>,
    comment_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    controller::authentication(&conn, cookies).await?;
    service::comments::update_comment_by_id(&conn, update_comment_json.0, comment_id).await?;

    let message = "Successfully edited comment!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: comment_id,
    };
    Ok(controller::create_success_response_ok(response))
}
