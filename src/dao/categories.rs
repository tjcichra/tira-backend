use crate::{
    models::{Category, ReturningId},
    TiraState,
};
use anyhow::{Context, Result};

/// DAO function for archiving category by id.
pub async fn archive_category_by_id(state: &TiraState, category_id: i64) -> Result<u64> {
    let result = sqlx::query("UPDATE categories SET archived = true WHERE id = $1")
        .bind(&category_id)
        .execute(&state.pool)
        .await?;
    Ok(result.rows_affected())
}

/// DAO function for creating a category.
///
/// Returns the id of the new category.
pub async fn create_category(
    state: &TiraState,
    category: Category,
    creator_id: i64,
) -> Result<i64> {
    let result = sqlx::query!(
        "INSERT INTO categories (name, description, creator_id) VALUES ($1, $2, $3) RETURNING id",
        category.name,
        category.description,
        creator_id
    )
    .fetch_all(&state.pool)
    .await?;

    let id = result
        .first()
        .context("could not get id when creating category")?
        .id;

    Ok(id)
}

/// DAO function for retrieving all categories.
pub async fn get_categories(
    state: &TiraState,
    filter_archived: Option<bool>,
) -> Result<Vec<Category>> {
    let categories = sqlx::query_as!(
        Category,
        "SELECT * FROM categories WHERE archived = $1",
        filter_archived.unwrap_or(false)
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(categories)
}

/// DAO function for retrieving a category by user id.
pub async fn get_category_by_id(state: &TiraState, user_id: i64) -> Result<Category> {
    let categories = sqlx::query_as!(
        Category,
        "SELECT *  FROM categories WHERE creator_id = $1",
        user_id
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(categories)
}
