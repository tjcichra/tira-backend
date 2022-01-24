use std::env;

use diesel::{PgConnection, Connection, RunQueryDsl};

use crate::models::User;
use dotenv::dotenv;

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

pub mod models;
pub mod schema;

fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[get("/")]
fn index() -> String {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users.first::<User>(&connection).expect("Could not find any user.");

    format!("{:?}", result)
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![index])
}
