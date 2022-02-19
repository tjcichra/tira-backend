use crate::controller::{self, TiraResponse};
use crate::models::{create::CreateCategory, Category};
use crate::service::categories;
use crate::TiraDbConn;
use rocket::serde::json::Json;

#[delete("/categories")]
pub async fn delete_categories_endpoint(conn: TiraDbConn) -> TiraResponse<usize> {
    controller::standardize_response(categories::delete_categories(conn).await)
}

#[delete("/categories/<category_id>")]
pub async fn delete_category_by_id_endpoint(
    conn: TiraDbConn,
    category_id: i64,
) -> TiraResponse<()> {
    controller::standardize_response(categories::delete_category_by_id(conn, category_id).await)
}

#[post("/categories", data = "<create_category_json>")]
pub async fn create_category_endpoint(
    conn: TiraDbConn,
    create_category_json: Json<CreateCategory>,
) -> TiraResponse<usize> {
    controller::standardize_response(
        categories::create_category(conn, create_category_json.0).await,
    )
}

#[get("/categories")]
pub async fn get_categories_endpoint(conn: TiraDbConn) -> TiraResponse<Vec<Category>> {
    controller::standardize_response(categories::get_categories(conn).await)
}

#[get("/categories/<category_id>")]
pub async fn get_category_by_id_endpoint(
    conn: TiraDbConn,
    category_id: i64,
) -> TiraResponse<Category> {
    controller::standardize_response(categories::get_category_by_id(conn, category_id).await)
}
