use std::env;

use diesel::{insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

use crate::models::User;

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_users() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    users
        .load::<User>(&connection)
        .expect("SQL for getting all users failed.")
}

pub fn get_user_by_id(user_id: i32) -> User {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    users
        .filter(id.eq(user_id))
        .first::<User>(&connection)
        .expect("Could not find any user.")
}

pub fn create_user(user: User) {
    use crate::schema::users::dsl::*;

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
