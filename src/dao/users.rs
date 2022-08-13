use crate::{
    models::{create::CreateUser, patch::UpdateUser, Assignment, Login, User},
    TiraDbConn,
};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for archiving a user by id.
pub async fn archive_user_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::update(users.filter(id.eq(user_id)))
            .set(archived.eq(true))
            .execute(c)
    })
    .await
}

/// DAO function for creating a user.
pub async fn create_user(conn: &TiraDbConn, user: CreateUser) -> QueryResult<i64> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(users)
            .values((&user, created.eq(Utc::now().naive_utc())))
            .returning(id)
            .get_result(c)
    })
    .await
}

/// DAO function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    conn: &TiraDbConn,
    user_id: i64,
) -> QueryResult<Vec<Assignment>> {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        assignments
            .filter(assignee_id.eq(user_id))
            .load::<Assignment>(c)
    })
    .await
}

/// DAO function for retrieving a user by id.
pub async fn get_user_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| users.filter(id.eq(user_id)).first::<User>(c))
        .await
}

/// DAO function for retrieving users by ids.
pub async fn get_users_by_ids(conn: &TiraDbConn, user_ids: Vec<i64>) -> QueryResult<Vec<User>> {
    use crate::schema::users;

    if user_ids.is_empty() {
        return Ok(vec![]);
    }

    conn.run(move |c| {
        let mut query = users::table.into_boxed();

        for user_id in user_ids {
            query = query.or_filter(users::id.eq(user_id));
        }

        query.load(c)
    })
    .await
}

/// DAO function for retrieving a user by username and password_hash.
pub async fn get_user_by_username_and_password(
    conn: &TiraDbConn,
    login_info: Login,
) -> QueryResult<User> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        users
            .filter(username.eq(login_info.username))
            .filter(password.eq(login_info.password))
            .first::<User>(c)
    })
    .await
}

/// DAO function for retrieving all users.
pub async fn get_users(conn: &TiraDbConn, filter_archived: Option<bool>) -> QueryResult<Vec<User>> {
    use crate::schema::users::dsl::*;

    match filter_archived {
        Some(filter_archived) => {
            conn.run(move |c| users.filter(archived.eq(filter_archived)).load::<User>(c))
                .await
        }
        None => conn.run(|c| users.load(c)).await,
    }
}

/// DAO function for updating a user by id.
pub async fn update_user_by_id(
    conn: &TiraDbConn,
    user: UpdateUser,
    user_id: i64,
) -> QueryResult<usize> {
    use crate::schema::users::dsl::*;

    conn.run(move |c| {
        diesel::update(users.filter(id.eq(user_id)))
            .set(user)
            .execute(c)
    })
    .await
}
