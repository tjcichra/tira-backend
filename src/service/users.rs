use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, result::Error};

use crate::{models::User, TiraDbConn};

pub async fn create_user(conn: TiraDbConn, user: User) -> QueryResult<usize> {
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
    })
    .await
}

pub async fn delete_user_by_id(conn: TiraDbConn, user_id: i32) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::delete(users.filter(id.eq(user_id)))
            .execute(c)
    })
    .await
}

pub async fn delete_users(conn: TiraDbConn) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    match conn.run(|c| {
        diesel::delete(users)
            .execute(c)
    })
    .await {
        Ok(1) => Ok(1),
        Ok(n) => Err(Error::NotFound),
        x => x
    }
}

pub async fn get_user_by_id(conn: TiraDbConn, user_id: i32) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        users
            .filter(id.eq(user_id))
            .first::<User>(c)
    })
    .await
}

pub async fn get_users(conn: TiraDbConn) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    conn.run(|c| users.load::<User>(c)).await
}
