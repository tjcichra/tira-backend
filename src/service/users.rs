use crate::{
    controller::{self, TiraErrorResponse},
    dao,
    models::{create::CreateUser, patch::UpdateUser, Assignment, User},
    service, TiraDbConn,
};

/// Service function for archiving a user by id.
pub async fn archive_user_by_id(conn: &TiraDbConn, user_id: i64) -> Result<(), TiraErrorResponse> {
    let users_archived = dao::users::archive_user_by_id(conn, user_id)
        .await
        .map_err(controller::convert)?;
    service::check_only_one_row_changed(users_archived)
}

/// Service function for creating a user.
pub async fn create_user(conn: &TiraDbConn, user: CreateUser) -> Result<i64, TiraErrorResponse> {
    dao::users::create_user(conn, user)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    conn: &TiraDbConn,
    user_id: i64,
) -> Result<Vec<Assignment>, TiraErrorResponse> {
    dao::users::get_assignments_by_user_id(conn, user_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving a user by id.
pub async fn get_user_by_id(conn: &TiraDbConn, user_id: i64) -> Result<User, TiraErrorResponse> {
    dao::users::get_user_by_id(conn, user_id)
        .await
        .map_err(controller::convert)
}

/// Service function for retrieving users by ids.
pub async fn get_users_by_ids(
    conn: &TiraDbConn,
    user_ids: Vec<i64>,
) -> Result<Vec<User>, TiraErrorResponse> {
    dao::users::get_users_by_ids(conn, user_ids)
        .await
        .map_err(controller::convert)
}

pub async fn get_users(
    conn: &TiraDbConn,
    filter_archived: Option<bool>,
) -> Result<Vec<User>, TiraErrorResponse> {
    dao::users::get_users(conn, filter_archived)
        .await
        .map_err(controller::convert)
}

/// Service function for updating a user by id.
pub async fn update_user_by_id(
    conn: &TiraDbConn,
    user: UpdateUser,
    user_id: i64,
) -> Result<(), TiraErrorResponse> {
    let users_updated = dao::users::update_user_by_id(conn, user, user_id)
        .await
        .map_err(controller::convert)?;
    service::check_only_one_row_changed(users_updated)
}
