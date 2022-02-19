use crate::{
    models::{create::CreateUser, User},
    service, TiraDbConn,
};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

pub async fn create_user(conn: TiraDbConn, user: CreateUser) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(users)
            .values(&user)
            .execute(c)
    })
    .await
}

pub async fn delete_user_by_id(conn: TiraDbConn, user_id: i64) -> QueryResult<()> {
    use crate::schema::users::dsl::*;

    let result = conn
        .run(move |c| diesel::delete(users.filter(id.eq(user_id))).execute(c))
        .await;

    service::check_only_one_row_changed(result)
}

pub async fn delete_users(conn: TiraDbConn) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    conn.run(|c| diesel::delete(users).execute(c)).await
}

pub async fn get_user_by_id(conn: TiraDbConn, user_id: i64) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| users.filter(id.eq(user_id)).first::<User>(c))
        .await
}

pub async fn get_users(conn: TiraDbConn) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    conn.run(|c| users.load::<User>(c)).await
}
