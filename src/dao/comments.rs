use diesel::{QueryDsl, QueryResult};

use crate::diesel::ExpressionMethods;
use crate::diesel::RunQueryDsl;
use crate::{models::patch::UpdateComment, TiraDbConn};

/// DAO function for updating a comment by id.
pub async fn update_comment_by_id(
    conn: &TiraDbConn,
    comment: UpdateComment,
    comment_id: i64,
) -> QueryResult<usize> {
    use crate::schema::comments::dsl::{comments, id};

    conn.run(move |c| {
        diesel::update(comments.filter(id.eq(comment_id)))
            .set(comment)
            .execute(c)
    })
    .await
}
