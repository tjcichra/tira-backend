use crate::{
    models::{create::CreateCategory, Category},
    TiraDbConn,
};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for archiving category by id.
pub async fn archive_category_by_id(conn: &TiraDbConn, category_id: i64) -> QueryResult<usize> {
    use crate::schema::categories;

    conn.run(move |c| {
        diesel::update(categories::table.filter(categories::id.eq(category_id)))
            .set(categories::archived.eq(true))
            .execute(c)
    })
    .await
}

/// DAO function for creating a category.
///
/// Returns the id of the new category.
pub async fn create_category(
    conn: &TiraDbConn,
    category: CreateCategory,
    creator_id_parameter: i64,
) -> QueryResult<i64> {
    use crate::schema::categories;

    conn.run(move |c| {
        diesel::insert_into(categories::table)
            .values((
                category,
                categories::creator_id.eq(creator_id_parameter),
                categories::created.eq(Utc::now().naive_utc()),
            ))
            .returning(categories::id)
            .get_result(c)
    })
    .await
}

/// DAO function for retrieving all categories.
pub async fn get_categories(
    conn: &TiraDbConn,
    filter_archived: Option<bool>,
) -> QueryResult<Vec<Category>> {
    use crate::schema::categories;

    match filter_archived {
        Some(filter_archived) => {
            conn.run(move |c| {
                categories::table
                    .filter(categories::archived.eq(filter_archived))
                    .load(c)
            })
            .await
        }
        None => conn.run(|c| categories::table.load(c)).await,
    }
}

/// DAO function for retrieving a category by user id.
pub async fn get_category_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<Category> {
    use crate::schema::categories;

    conn.run(move |c| {
        categories::table
            .filter(categories::id.eq(user_id))
            .first(c)
    })
    .await
}
