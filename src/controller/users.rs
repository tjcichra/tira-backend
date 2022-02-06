use crate::service::users;
use crate::TiraDbConn;
use crate::User;
use rocket::serde::json::Json;

#[post("/users", data = "<user_json>")]
pub async fn create_user_endpoint(conn: TiraDbConn, user_json: Json<User>) {
    users::create_user(conn, user_json.0).await;
}

#[get("/users")]
pub async fn get_users_endpoint(conn: TiraDbConn) -> Result<Json<Vec<User>>, Json<String>> {
    let x = users::get_users(conn).await;
    let x = x.map(|x| Json(x));
    let x = x.map_err(|x| Json(x.to_string()));

    x
}

#[get("/users/<user_id>")]
pub async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) -> Json<User> {
    Json(users::get_user_by_id(conn, user_id).await)
}

#[delete("/users")]
pub async fn delete_users_endpoint(conn: TiraDbConn) {
    users::delete_users(conn).await;
}

#[delete("/users/<user_id>")]
pub async fn delete_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) {
    users::delete_user_by_id(conn, user_id).await;
}
