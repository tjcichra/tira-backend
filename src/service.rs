use std::env;

use diesel::{
    delete, insert_into, Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl,
};

use crate::{models::User, TiraDbConn};

pub fn establish_connection() -> PgConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub async fn create_user(conn: TiraDbConn, user: User) {
    use crate::schema::users::dsl::*;

    conn.run(|c| {
        insert_into(users)
            .values((
                username.eq(user.username),
                password.eq(user.password),
                email_address.eq(user.email_address),
                first_name.eq(user.first_name),
                last_name.eq(user.last_name),
            ))
            .execute(c)
            .expect("Error with inserting user")
    })
    .await;
}

pub async fn get_users(conn: TiraDbConn) -> Vec<User> {
    use crate::schema::users::dsl::*;

    conn.run(|c| {
        users
            .load::<User>(c)
            .expect("SQL for getting all users failed.")
    })
    .await
}

pub async fn get_user_by_id(conn: TiraDbConn, user_id: i32) -> User {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        users
            .filter(id.eq(user_id))
            .first::<User>(c)
            .expect("Could not find any user.")
    })
    .await
}

pub async fn delete_users(conn: TiraDbConn) {
    use crate::schema::users::dsl::*;

    conn.run(|c| {
        delete(users)
            .execute(c)
            .expect("Failed to delete users table");
    })
    .await;
}

pub async fn delete_user_by_id(conn: TiraDbConn, user_id: i32) {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        delete(users.filter(id.eq(user_id)))
            .execute(c)
            .expect("Failed to delete users table")
    })
    .await;
}
