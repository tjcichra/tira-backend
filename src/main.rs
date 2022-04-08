use crate::controller::{categories, sessions, tickets, users};
use diesel::PgConnection;
use dotenv::dotenv;
use rocket::{figment::{
    map,
    value::{Map, Value},
}, fairing::{Fairing, Info, Kind}, http::Header, Request, Response};
use rocket_sync_db_pools::database;
use std::env;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod controller;
mod dao;
mod models;
mod schema;
mod service;

#[database("tira_db")]
pub struct TiraDbConn(PgConnection);

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "false"));
    }
}

// The point where the program first starts
#[launch]
fn rocket() -> _ {
    // Import environment variables from .env file
    dotenv().ok();

    // Get database URL from environment variables and add it to programmatic rocket config
    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10_i32.into()
    };
    let figment = rocket::Config::figment().merge(("databases", map!["tira_db" => db]));

    // Set up rocket's customizations and endpoints.
    rocket::custom(figment)
        .attach(TiraDbConn::fairing())
        .attach(CORS)
        .mount(
            "/",
            routes![
                categories::archive_category_by_id_endpoint,
                categories::create_category_endpoint,
                categories::get_categories_endpoint,
                categories::get_category_by_id_endpoint,
                sessions::login_endpoint,
                tickets::create_assignment_by_ticket_id_endpoint,
                tickets::create_comment_by_ticket_id_endpoint,
                tickets::create_ticket_endpoint,
                tickets::get_assignments_by_ticket_id_endpoint,
                tickets::get_comments_by_ticket_id_endpoint,
                tickets::get_ticket_by_id_endpoint,
                tickets::get_tickets_endpoint,
                users::archive_user_by_id_endpoint,
                users::create_user_endpoint,
                users::get_user_by_id_endpoint,
                users::get_users_endpoint,
            ],
        )
        .register("/", catchers![controller::not_found])
}
