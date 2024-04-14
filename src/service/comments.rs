use crate::{dao, models::patch::UpdateComment, service, TiraState};
use anyhow::Result;

/// Service function for updating a comment by id.
pub async fn update_comment_by_id(
    state: &TiraState,
    comment: UpdateComment,
    comment_id: i64,
) -> Result<()> {
    let comments_updated = dao::comments::update_comment_by_id(state, comment, comment_id).await?;
    service::check_only_one_row_changed(comments_updated)
}
