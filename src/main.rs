use diesel::PgConnection;
use rocket_sync_db_pools::database;
use service::{create_user, delete_user_by_id, delete_users, get_user_by_id, get_users};

use crate::models::User;
use dotenv::dotenv;
use rocket::serde::json::Json;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod service;

#[post("/users", data = "<user_json>")]
async fn create_user_endpoint(conn: TiraDbConn, user_json: Json<User>) {
    create_user(conn, user_json.0).await;
}

#[get("/users")]
async fn get_users_endpoint(conn: TiraDbConn) -> Json<Vec<User>> {
    Json(get_users(conn).await)
}

#[get("/users/<user_id>")]
async fn get_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) -> Json<User> {
    Json(get_user_by_id(conn, user_id).await)
}

#[delete("/users")]
async fn delete_users_endpoint(conn: TiraDbConn) {
    delete_users(conn).await;
}

#[delete("/users/<user_id>")]
async fn delete_user_by_id_endpoint(conn: TiraDbConn, user_id: i32) {
    delete_user_by_id(conn, user_id).await;
}

#[database("r2d2_url_thingy")]
pub struct TiraDbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        // .attach(CORS)
        .attach(TiraDbConn::fairing())
        .mount(
            "/",
            routes![
                create_user_endpoint,
                get_users_endpoint,
                get_user_by_id_endpoint,
                delete_users_endpoint,
                delete_user_by_id_endpoint
            ],
        )
}
