use crate::models::User;
use crate::service::users;
use crate::TiraDbConn;
use rocket::http::CookieJar;
use rocket::serde::json::Json;

use crate::controller;

#[post("/users", data = "<user_json>")]
pub async fn create_user_endpoint(conn: TiraDbConn, user_json: Json<User>) {
    users::create_user(conn, user_json.0).await;
}

#[delete("/users/<user_id>")]
pub async fn delete_user_by_id_endpoint(conn: TiraDbConn, user_id: i64) {
    users::delete_user_by_id(conn, user_id).await;
}

#[delete("/users")]
pub async fn delete_users_endpoint(conn: TiraDbConn) {
    users::delete_users(conn).await;
}

#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i64) -> super::TiraResponse<User> {
    controller::standardize_response(users::get_user_by_id(conn, user_id).await)
}

#[get("/users")]
pub async fn get_users_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
) -> super::TiraResponse<Vec<User>> {
    println!("{}", cookies.get("tirauth").unwrap().value());
    controller::standardize_response(users::get_users(conn).await)
}
