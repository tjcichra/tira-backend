use crate::{
    models::{create::CreateUser, patch::UpdateUser, Assignment, Login, User},
    TiraDbConn,
};
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for archiving a user by id.
pub async fn archive_user_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<usize> {
    use crate::schema::users;

    conn.run(move |c| {
        diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(users::archived.eq(true))
            .execute(c)
    })
    .await
}

/// DAO function for creating a user.
pub async fn create_user(conn: &TiraDbConn, user: CreateUser) -> QueryResult<i64> {
    use crate::schema::users;

    conn.run(move |c| {
        diesel::insert_into(users::table)
            .values((&user, users::created.eq(Utc::now().naive_utc())))
            .returning(users::id)
            .get_result(c)
    })
    .await
}

/// DAO function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    conn: &TiraDbConn,
    user_id: i64,
) -> QueryResult<Vec<Assignment>> {
    use crate::schema::assignments;

    conn.run(move |c| {
        assignments::table
            .filter(assignments::assignee_id.eq(user_id))
            .load::<Assignment>(c)
    })
    .await
}

/// DAO function for retrieving a user by id.
pub async fn get_user_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<User> {
    use crate::schema::users;

    conn.run(move |c| users::table.filter(users::id.eq(user_id)).first::<User>(c))
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
    use crate::schema::users;

    conn.run(move |c| {
        users::table
            .filter(users::username.eq(login_info.username))
            .filter(users::password.eq(login_info.password))
            .first::<User>(c)
    })
    .await
}

/// DAO function for retrieving all users.
pub async fn get_users(conn: &TiraDbConn, filter_archived: Option<bool>) -> QueryResult<Vec<User>> {
    match filter_archived {
        Some(filter_archived) => {
            conn.run(move |c| {
                users::table
                    .filter(users::archived.eq(filter_archived))
                    .load::<User>(c)
            })
            .await
            
            conn.run(move |c| {
                c.execute(
                    "SELECT  users SET username = $1, password = $2, email_address = $3, first_name = $4, last_name = $5, profile_picture_url = $6, archived = $7 WHERE id = $8",
                    &[&user.username, &user.password, &user.email_address, &user.first_name, &user.last_name, &user.profile_picture_url, &user.archived, &user_id]
                );
            }).await
        }
        None => conn.run(|c| users::table.load(c)).await,
    }
}

/// DAO function for updating a user by id.
pub async fn update_user_by_id(
    conn: &TiraDbConn,
    user: UpdateUser,
    user_id: i64,
) -> QueryResult<usize> {
    conn.run(move |c| {
        c.execute(
            "UPDATE users SET username = $1, password = $2, email_address = $3, first_name = $4, last_name = $5, profile_picture_url = $6, archived = $7 WHERE id = $8",
            &[&user.username, &user.password, &user.email_address, &user.first_name, &user.last_name, &user.profile_picture_url, &user.archived, &user_id]
        );
    }).await
}
