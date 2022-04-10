use crate::{
    controller::TiraMessage,
    dao::{self, users},
    models::{create::CreateUser, Assignment, User},
    service, TiraDbConn,
};
use diesel::QueryResult;

/// Service function for archiving a user by id.
pub async fn archive_user_by_id(conn: &TiraDbConn, user_id: i64) -> Result<(), TiraMessage> {
    let users_archived = users::archive_user_by_id(conn, user_id).await;
    service::check_only_one_row_changed(users_archived)
}

/// Service function for creating a user.
pub async fn create_user(conn: &TiraDbConn, user: CreateUser) -> Result<(), TiraMessage> {
    let users_created = users::create_user(conn, user).await;
    service::check_only_one_row_changed(users_created)
}

/// Service function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    conn: &TiraDbConn,
    user_id: i64,
) -> QueryResult<Vec<Assignment>> {
    dao::users::get_assignments_by_user_id(conn, user_id).await
}

/// Service function for retrieving a user by id.
pub async fn get_user_by_id(conn: &TiraDbConn, user_id: i64) -> QueryResult<User> {
    users::get_user_by_id(conn, user_id).await
}

pub async fn get_users(conn: &TiraDbConn, filter_archived: Option<bool>) -> QueryResult<Vec<User>> {
    users::get_users(conn, filter_archived).await
}
