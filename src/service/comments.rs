use crate::{
    controller::{self, TiraErrorResponse},
    dao,
    models::patch::UpdateComment,
    service, TiraDbConn,
};

/// Service function for updating a comment by id.
pub async fn update_comment_by_id(
    conn: &TiraDbConn,
    comment: UpdateComment,
    comment_id: i64,
) -> Result<(), TiraErrorResponse> {
    let comments_updated = dao::comments::update_comment_by_id(conn, comment, comment_id)
        .await
        .map_err(controller::convert)?;
    service::check_only_one_row_changed(comments_updated)
}
