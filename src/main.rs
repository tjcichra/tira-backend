use service::{create_user, get_user_by_id, get_users, delete_users, delete_user_by_id};

use crate::models::User;
use dotenv::dotenv;
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    serde::json::Json,
    Request, Response,
};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;
pub mod service;

#[post("/users", data = "<user_json>")]
fn create_user_endpoint(user_json: Json<User>) {
    create_user(user_json.0);
}

#[get("/users")]
fn get_users_endpoint() -> Json<Vec<User>> {
    Json(get_users())
}

#[get("/users/<user_id>")]
fn get_user_by_id_endpoint(user_id: i32) -> Json<User> {
    Json(get_user_by_id(user_id))
}

#[delete("/users")]
fn delete_users_endpoint() {
    delete_users();
}

#[delete("/users/<user_id>")]
fn delete_user_by_id_endpoint(user_id: i32) {
    delete_user_by_id(user_id);
}

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build().attach(CORS).mount(
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
