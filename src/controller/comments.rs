use super::TiraError;
use crate::models::patch::UpdateComment;
use crate::models::success::AlteredResourceResponse;
use crate::service;
use crate::TiraState;
use anyhow::Result;
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::Json;

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
pub async fn patch_comment_by_id_endpoint(
    State(state): State<TiraState>,
    Path(comment_id): Path<i64>,
    Json(comment): Json<UpdateComment>,
) -> Result<Response, TiraError> {
    service::comments::update_comment_by_id(&state, comment, comment_id).await?;
    let message = "Successfully edited comment!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: comment_id,
    };
    Ok(Json(response).into_response())
}
