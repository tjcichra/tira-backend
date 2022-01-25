use std::env;

use diesel::{insert_into, Connection, PgConnection, QueryDsl, RunQueryDsl};

use crate::diesel::ExpressionMethods;
use crate::models::User;
use dotenv::dotenv;
use rocket::serde::json::Json;

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

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users
        .load::<User>(&connection)
        .expect("SQL for getting all users failed.");

    Json(result)
}

#[get("/users/<user_id>")]
fn get_user_by_id(user_id: i32) -> Json<User> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let result = users
        .filter(id.eq(user_id))
        .first::<User>(&connection)
        .expect("Could not find any user.");

    Json(result)
}

#[post("/users", data = "<user_json>")]
fn create_user(user_json: Json<User>) {
    use crate::schema::users::dsl::*;

    let user = user_json.0;

    let connection = establish_connection();
    insert_into(users)
        .values((
            username.eq(user.username),
            password.eq(user.password),
            email_address.eq(user.email_address),
            first_name.eq(user.first_name),
            last_name.eq(user.last_name),
        ))
        .execute(&connection)
        .expect("Error with inserting user");
}

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/", routes![get_users, get_user_by_id, create_user])
}
