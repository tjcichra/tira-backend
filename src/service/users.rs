use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

use crate::{models::User, TiraDbConn};

pub async fn create_user(conn: TiraDbConn, user: User) {
    use crate::schema::users::dsl::*;

    conn.run(|c| {
        diesel::insert_into(users)
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

pub async fn get_users(conn: TiraDbConn) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    conn.run(|c| users.load::<User>(c)).await
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
        diesel::delete(users)
            .execute(c)
            .expect("Failed to delete users table");
    })
    .await;
}

pub async fn delete_user_by_id(conn: TiraDbConn, user_id: i32) {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(c)
            .expect("Failed to delete users table")
    })
    .await;
}
