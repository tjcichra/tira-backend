use super::TiraError;
use crate::models::success::{AlteredResourceResponse, StandardResponse};
use crate::models::{Category, Session};
use crate::service::categories;
use crate::TiraState;
use anyhow::Result;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ArchiveCategoryQueryParams {
    category_id: i64,
}

/// Endpoint for archiving a specific category.
///
/// Requires authentication.
///
/// **DELETE /categories/<category_id>**

pub async fn archive_category_by_id_endpoint(
    State(state): State<TiraState>,
    query_params: Query<ArchiveCategoryQueryParams>,
) -> Result<Response, TiraError> {
    categories::archive_category_by_id(&state, query_params.category_id).await?;

    let message = format!(
        "Successfully archived category with id {}!",
        query_params.category_id
    );
    let response = StandardResponse { message };
    Ok(Json(response).into_response())
}

/// Endpoint for creating a category.
///
/// Requires authentication.
///
/// **POST /categories**
///
/// Example JSON Body:
///
/// {
///     "name": "testname",
///     "description": "testdescription"
/// }
pub async fn create_category_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
    Json(category): Json<Category>,
) -> Result<Response, TiraError> {
    let category_id = categories::create_category(&state, category, session.user_id).await?;

    let message = format!("Successfully created category with id {}", category_id);
    let response = AlteredResourceResponse {
        message,
        id: category_id,
    };
    Ok((StatusCode::CREATED, Json(response)).into_response())
}

#[derive(Deserialize)]
pub struct GetCategoryQueryParams {
    archived: Option<bool>,
}

/// Endpoint for retrieving every category.
///
/// **GET /categories**
///
/// Query Parameters:
///
/// archived: Used to filter categories that are archived or not. Takes a boolean value. (optional)

pub async fn get_categories_endpoint(
    State(state): State<TiraState>,
    query_params: Query<GetCategoryQueryParams>,
) -> Result<Response, TiraError> {
    let categories = categories::get_categories(&state, query_params.archived).await?;
    Ok(Json(categories).into_response())
}

/// Endpoint for retrieving a category.
///
/// **GET /categories/<category_id>**

pub async fn get_category_by_id_endpoint(
    State(state): State<TiraState>,
    Path(category_id): Path<i64>,
) -> Result<Response, TiraError> {
    let category = categories::get_category_by_id(&state, category_id).await?;
    Ok(Json(category).into_response())
}
