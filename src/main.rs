use controller::categories;
use diesel::PgConnection;
use rocket_sync_db_pools::database;

use crate::controller::{tickets, users};
use crate::models::User;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod controller;
pub mod models;
pub mod schema;
pub mod service;

#[database("r2d2_url_thingy")]
pub struct TiraDbConn(PgConnection);

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(TiraDbConn::fairing())
        .mount(
            "/",
            routes![
                categories::create_category_endpoint,
                categories::delete_categories_endpoint,
                categories::delete_category_by_id_endpoint,
                categories::get_categories_endpoint,
                categories::get_category_by_id_endpoint,
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
