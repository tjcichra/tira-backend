use crate::{models::patch::UpdateComment, TiraState};

/// DAO function for updating a comment by id.
pub async fn update_comment_by_id(
    state: &TiraState,
    comment: UpdateComment,
    comment_id: i64,
) -> anyhow::Result<u64> {
    let result = sqlx::query("UPDATE comments SET comment = $1 WHERE id = $2")
        .bind(comment.content)
        .bind(comment_id)
        .execute(&state.pool)
        .await?;
    Ok(result.rows_affected())
}
