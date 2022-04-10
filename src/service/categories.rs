use crate::{
    controller::TiraMessage,
    dao::categories,
    models::{create::CreateCategory, Category},
    service, TiraDbConn,
};
use diesel::result::Error as QueryError;

/// Service function for archiving category by id.
pub async fn archive_category_by_id(
    conn: &TiraDbConn,
    category_id: i64,
) -> Result<(), TiraMessage> {
    let categories_archived = categories::archive_category_by_id(conn, category_id).await;
    service::check_only_one_row_changed(categories_archived)
}

/// Service function for creating a category.
pub async fn create_category(
    conn: &TiraDbConn,
    category: CreateCategory,
    creator_id: i64,
) -> Result<(), TiraMessage> {
    let categories_created = categories::create_category(conn, category, creator_id).await;
    service::check_only_one_row_changed(categories_created)
}

/// Service function for retrieving all categories.
pub async fn get_categories(
    conn: &TiraDbConn,
    filter_archived: Option<bool>,
) -> Result<Vec<Category>, QueryError> {
    categories::get_categories(conn, filter_archived).await
}

/// Service function for retrieving a category by user id.
pub async fn get_category_by_id(conn: &TiraDbConn, user_id: i64) -> Result<Category, QueryError> {
    categories::get_category_by_id(conn, user_id).await
}
