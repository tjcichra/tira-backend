use crate::{
    dao::{self, categories},
    models::Category,
    service, TiraState,
};
use anyhow::Result;

/// Service function for archiving category by id.
pub async fn archive_category_by_id(state: &TiraState, category_id: i64) -> Result<()> {
    let categories_archived = categories::archive_category_by_id(state, category_id).await?;
    service::check_only_one_row_changed(categories_archived)
}

/// Service function for creating a category.
pub async fn create_category(
    state: &TiraState,
    category: Category,
    creator_id: i64,
) -> Result<i64> {
    let category = dao::categories::create_category(state, category, creator_id).await?;
    Ok(category)
}

/// Service function for retrieving all categories.
pub async fn get_categories(
    state: &TiraState,
    filter_archived: Option<bool>,
) -> Result<Vec<Category>> {
    let categories = dao::categories::get_categories(state, filter_archived).await?;
    Ok(categories)
}

/// Service function for retrieving a category by category id.
pub async fn get_category_by_id(state: &TiraState, category_id: i64) -> Result<Category> {
    let category = dao::categories::get_category_by_id(state, category_id).await?;
    Ok(category)
}
