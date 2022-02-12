use crate::controller::{categories, tickets, users};
use diesel::PgConnection;
use dotenv::dotenv;
use rocket::figment::{
    map,
    value::{Map, Value},
};
use rocket_sync_db_pools::database;
use std::env;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod controller;
mod models;
mod schema;
mod service;

#[database("tira_db")]
pub struct TiraDbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("Environment variable DATABASE_URL not found.");
    let db: Map<_, Value> = map! {
        "url" => db_url.into(),
        "pool_size" => 10_i32.into()
    };
    let figment = rocket::Config::figment().merge(("databases", map!["tira_db" => db]));

    rocket::custom(figment)
        .attach(TiraDbConn::fairing())
        .mount(
            "/",
            routes![
                categories::create_category_endpoint,
                categories::delete_categories_endpoint,
                categories::delete_category_by_id_endpoint,
                categories::get_categories_endpoint,
                categories::get_category_by_id_endpoint,
                controller::login_endpoint,
                tickets::create_assignment_by_ticket_id_endpoint,
                tickets::create_comment_endpoint,
                tickets::create_ticket_endpoint,
                tickets::delete_ticket_by_id_endpoint,
                tickets::delete_tickets_endpoint,
                tickets::get_assignments_by_ticket_id_endpoint,
                tickets::get_comments_by_ticket_id_endpoint,
                tickets::get_ticket_by_id_endpoint,
                tickets::get_tickets_endpoint,
                users::create_user_endpoint,
                users::delete_user_by_id_endpoint,
                users::delete_users_endpoint,
                users::get_user_by_id_endpoint,
                users::get_users_endpoint,
            ],
        )
        .register("/", catchers![controller::not_found])
}
