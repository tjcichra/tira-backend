use crate::service::emails::handle_emails;
use crate::service::emails::Email;
use diesel::PgConnection;
use dotenv::dotenv;
use rocket::{
    fairing::{Fairing, Info, Kind},
    figment::{
        map,
        value::{Map, Value},
    },
    http::Header,
    Request, Response,
};
use rocket_sync_db_pools::database;
use std::env;
use std::sync::mpsc;

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod controller;
mod dao;
mod models;
mod schema;
mod service;

pub struct TiraState {
    email_tx: mpsc::SyncSender<Email>,
}

#[database("tira_db")]
pub struct TiraDbConn(PgConnection);

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> rocket::fairing::Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            "http://localhost:3000",
        ));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PUT, DELETE, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
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

    let (email_tx, email_rx) = mpsc::sync_channel(512);
    // Listen for emails on the email queue
    rocket::tokio::task::spawn(async move {
        handle_emails(email_rx);
    });

    // Set up rocket's customizations and endpoints.
    rocket::custom(figment)
        .attach(TiraDbConn::fairing())
        .attach(CORS)
        .manage(TiraState { email_tx })
        .mount(
            "/",
            routes![
                controller::assignments::get_assignments_endpoint,
                controller::categories::archive_category_by_id_endpoint,
                controller::categories::create_category_endpoint,
                controller::categories::get_categories_endpoint,
                controller::categories::get_category_by_id_endpoint,
                controller::comments::patch_comment_by_id_endpoint,
                controller::images::retrieve_image_endpoint,
                controller::images::upload_image_endpoint,
                controller::sessions::login_endpoint,
                controller::sessions::logout_endpoint,
                controller::tickets::create_assignment_by_ticket_id_endpoint,
                controller::tickets::create_comment_by_ticket_id_endpoint,
                controller::tickets::create_ticket_endpoint,
                controller::tickets::get_assignments_by_ticket_id_endpoint,
                controller::tickets::get_comments_by_ticket_id_endpoint,
                controller::tickets::get_ticket_by_id_endpoint,
                controller::tickets::get_tickets_endpoint,
                controller::tickets::patch_ticket_by_id_endpoint,
                controller::users::archive_user_by_id_endpoint,
                controller::users::create_user_endpoint,
                controller::users::get_assignments_by_user_id_endpoint,
                controller::users::get_current_user_endpoint,
                controller::users::get_user_by_id_endpoint,
                controller::users::get_users_endpoint,
                controller::users::patch_user_by_id_endpoint
            ],
        )
        .register("/", catchers![controller::not_found])
}
