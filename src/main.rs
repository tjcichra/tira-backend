use service::{create_user, get_user_by_id, get_users};

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

#[get("/users")]
fn get_users_endpoint() -> Json<Vec<User>> {
    Json(get_users())
}

#[get("/users/<user_id>")]
fn get_user_by_id_endpoint(user_id: i32) -> Json<User> {
    Json(get_user_by_id(user_id))
}

#[post("/users", data = "<user_json>")]
fn create_user_endpoint(user_json: Json<User>) {
    create_user(user_json.0);
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount(
        "/",
        routes![get_users_endpoint, get_user_by_id_endpoint, create_user_endpoint],
    )
}
