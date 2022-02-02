use crate::models::{Category, CreateCategory};
use crate::service::categories::{
    create_category, delete_categories, delete_category_by_id, get_categories, get_category_by_id,
};
use crate::TiraDbConn;
use rocket::serde::json::Json;

#[post("/categories", data = "<create_category_json>")]
pub async fn create_category_endpoint(
    conn: TiraDbConn,
    create_category_json: Json<CreateCategory>,
) {
    create_category(conn, create_category_json.0).await;
}

#[get("/categories")]
pub async fn get_categories_endpoint(conn: TiraDbConn) -> Json<Vec<Category>> {
    Json(get_categories(conn).await)
}

#[get("/categories/<category_id>")]
pub async fn get_category_by_id_endpoint(conn: TiraDbConn, category_id: i32) -> Json<Category> {
    Json(get_category_by_id(conn, category_id).await)
}

#[delete("/categories")]
pub async fn delete_categories_endpoint(conn: TiraDbConn) {
    delete_categories(conn).await;
}

#[delete("/categories/<category_id>")]
pub async fn delete_category_by_id_endpoint(conn: TiraDbConn, category_id: i32) {
    delete_category_by_id(conn, category_id).await;
}
