use crate::service::users::create_user;
use crate::service::users::delete_user_by_id;
use crate::service::users::delete_users;
use crate::service::users::get_user_by_id;
use crate::service::users::get_users;
use crate::TiraDbConn;
use crate::User;
use rocket::serde::json::Json;

#[post("/users", data = "<user_json>")]
pub async fn create_user_endpoint(conn: TiraDbConn, user_json: Json<User>) {
    create_user(conn, user_json.0).await;
}

#[get("/users")]
pub async fn get_users_endpoint(conn: TiraDbConn) -> Json<Vec<User>> {
    Json(get_users(conn).await)
}

#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) -> Json<User> {
    Json(get_user_by_id(conn, user_id).await)
}

#[delete("/users")]
pub async fn delete_users_endpoint(conn: TiraDbConn) {
    delete_users(conn).await;
}

#[delete("/users/<user_id>")]
pub async fn delete_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) {
    delete_user_by_id(conn, user_id).await;
}
