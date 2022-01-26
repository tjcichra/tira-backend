use diesel::PgConnection;
use rocket_sync_db_pools::database;

use crate::controller::users::{
    create_user_endpoint, delete_user_by_id_endpoint, delete_users_endpoint,
    get_user_by_id_endpoint, get_users_endpoint,
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

    rocket::build().attach(TiraDbConn::fairing()).mount(
        "/",
        routes![
            create_user_endpoint,
            get_users_endpoint,
            get_user_by_id_endpoint,
            delete_users_endpoint,
            delete_user_by_id_endpoint
        ]
    )
}
