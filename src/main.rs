use controller::categories::{
    create_category_endpoint, delete_categories_endpoint, delete_category_by_id_endpoint,
    get_categories_endpoint, get_category_by_id_endpoint,
};
use controller::not_found;
use diesel::PgConnection;
use rocket_sync_db_pools::database;

use crate::controller::users::{
    create_user_endpoint, delete_user_by_id_endpoint, delete_users_endpoint,
    get_user_by_id_endpoint, get_users_endpoint,
};
use crate::controller::tickets::{
    create_ticket_endpoint, delete_ticket_by_id_endpoint, delete_tickets_endpoint,
    get_ticket_by_id_endpoint, get_tickets_endpoint,
};
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
                create_user_endpoint,
                get_users_endpoint,
                get_user_by_id_endpoint,
                delete_users_endpoint,
                delete_user_by_id_endpoint,
                create_category_endpoint,
                get_categories_endpoint,
                get_category_by_id_endpoint,
                delete_categories_endpoint,
                delete_category_by_id_endpoint,
                create_ticket_endpoint,
                get_tickets_endpoint,
                get_ticket_by_id_endpoint,
                delete_tickets_endpoint,
                delete_ticket_by_id_endpoint,
            ],
        )
        .register("/", catchers![not_found])
}
