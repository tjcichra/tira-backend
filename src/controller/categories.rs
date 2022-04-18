use crate::controller::{self, TiraResponse};
use crate::models::success::{StandardResponse, AlteredResourceResponse};
use crate::models::{create::CreateCategory, Category};
use crate::service::categories;
use crate::TiraDbConn;
use rocket::http::{CookieJar, Status};
use rocket::serde::json::Json;

/// Endpoint for archiving a specific category.
///
/// Requires authentication.
///
/// **DELETE /categories/<category_id>**
#[delete("/categories/<category_id>")]
pub async fn archive_category_by_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    category_id: i64,
) -> TiraResponse<StandardResponse> {
    controller::authentication(&conn, cookies).await?;
    categories::archive_category_by_id(&conn, category_id).await?;

    let message = format!("Successfully archived category with id {}!", category_id);
    let response = StandardResponse { message };
    Ok(controller::create_success_response_ok(response))
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
#[post("/categories", data = "<create_category_json>")]
pub async fn create_category_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_category_json: Json<CreateCategory>,
) -> TiraResponse<AlteredResourceResponse> {
    let user_id = controller::authentication(&conn, cookies).await?;
    let category_id = categories::create_category(&conn, create_category_json.0, user_id).await?;

    let message = format!("Successfully created category with id {}", category_id);
    let response = AlteredResourceResponse { message, id: category_id };
    Ok(controller::create_success_response(Status::Created, response))
}

/// Endpoint for retrieving every category.
///
/// **GET /categories**
///
/// Query Parameters:
///
/// archived: Used to filter categories that are archived or not. Takes a boolean value. (optional)
#[get("/categories?<archived>")]
pub async fn get_categories_endpoint(
    conn: TiraDbConn,
    archived: Option<bool>,
) -> TiraResponse<Vec<Category>> {
    let categories = categories::get_categories(&conn, archived).await?;
    Ok(controller::create_success_response_ok(categories))
}

/// Endpoint for retrieving a category.
///
/// **GET /categories/<category_id>**
#[get("/categories/<category_id>")]
pub async fn get_category_by_id_endpoint(
    conn: TiraDbConn,
    category_id: i64,
) -> TiraResponse<Category> {
    let category = categories::get_category_by_id(&conn, category_id).await?;
    Ok(controller::create_success_response_ok(category))
}
