use crate::{
    models::{patch::UpdateUser, Assignment, Login, User},
    TiraState,
};
use anyhow::Result;
use sqlx::QueryBuilder;

/// DAO function for archiving a user by id.
pub async fn archive_user_by_id(state: &TiraState, user_id: i64) -> Result<u64> {
    let result = sqlx::query!("UPDATE users SET archived = true WHERE id = $1", user_id)
        .execute(&state.pool)
        .await?;
    Ok(result.rows_affected())
}

/// DAO function for creating a user.
pub async fn create_user(state: &TiraState, user: User) -> Result<i64> {
    let result =  sqlx::query!("INSERT INTO users (username, password, email_address, first_name, last_name, profile_picture_url) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id",
    user.username,
    user.password,
    user.email_address,
    user.first_name,
    user.last_name,
    user.profile_picture_url
    )
.fetch_one(&state.pool).await?;
    Ok(result.id)
}

/// DAO function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    state: &TiraState,
    user_id: i64,
) -> Result<Vec<Assignment>> {
    let assignments = sqlx::query_as!(
        Assignment,
        "SELECT * FROM assignments WHERE assignee_id = $1",
        user_id
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(assignments)
}

/// DAO function for retrieving a user by id.
pub async fn get_user_by_id(state: &TiraState, user_id: i64) -> Result<User> {
    let users = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_one(&state.pool)
        .await?;
    Ok(users)
}

/// DAO function for retrieving users by ids.
pub async fn get_users_by_ids(state: &TiraState, user_ids: Vec<i64>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE id IN (SELECT unnest($1::integer[]))",
        &user_ids,
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(users)
}

/// DAO function for retrieving a user by username and password_hash.
pub async fn get_user_by_username_and_password(state: &TiraState, login: Login) -> Result<User> {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE username = $1 and password = $2",
        login.username,
        login.password,
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(users)
}

/// DAO function for retrieving all users.
pub async fn get_users(state: &TiraState, filter_archived: Option<bool>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE archived = $1",
        filter_archived.unwrap_or(false)
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(users)
}

/// DAO function for updating a user by id.
pub async fn update_user_by_id(state: &TiraState, user: UpdateUser, user_id: i64) -> Result<u64> {
    let mut query = QueryBuilder::new("UPDATE users SET ");

    if let Some(username) = user.username {
        query.push("username = ");
        query.push_bind(username);
    }
    if let Some(password) = user.password {
        query.push("password = ");
        query.push_bind(password);
    }
    if let Some(email_address) = user.email_address {
        query.push("email_address = ");
        query.push_bind(email_address);
    }
    if let Some(first_name) = user.first_name {
        query.push("first_name = ");
        query.push_bind(first_name);
    }
    if let Some(last_name) = user.last_name {
        query.push("last_name = ");
        query.push_bind(last_name);
    }
    if let Some(profile_picture_url) = user.profile_picture_url {
        query.push("profile_picture_url = ");
        query.push_bind(profile_picture_url);
    }
    if let Some(archived) = user.archived {
        query.push("archived = ");
        query.push_bind(archived);
    }

    query.push("WHERE id = ");
    query.push_bind(user_id);

    let result = query.build().execute(&state.pool).await?;

    Ok(result.rows_affected())
}
