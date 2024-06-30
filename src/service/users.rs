use crate::{
    dao,
    models::{patch::UpdateUser, Assignment, User},
    service, TiraState,
};
use anyhow::Result;

/// Service function for archiving a user by id.
pub async fn archive_user_by_id(state: &TiraState, user_id: i64) -> Result<()> {
    let users_archived = dao::users::archive_user_by_id(state, user_id).await?;
    service::check_only_one_row_changed(users_archived)
}

/// Service function for creating a user.
pub async fn create_user(state: &TiraState, user: User) -> Result<i64> {
    dao::users::create_user(state, user).await
}

/// Service function for retrieving all assignments for a user.
pub async fn get_assignments_by_user_id(
    state: &TiraState,
    user_id: i64,
) -> Result<Vec<Assignment>> {
    dao::users::get_assignments_by_user_id(state, user_id).await
}

/// Service function for retrieving a user by id.
pub async fn get_user_by_id(state: &TiraState, user_id: i64) -> Result<User> {
    dao::users::get_user_by_id(state, user_id).await
}

/// Service function for retrieving users by ids.
pub async fn get_users_by_ids(state: &TiraState, user_ids: Vec<i64>) -> Result<Vec<User>> {
    dao::users::get_users_by_ids(state, user_ids).await
}

pub async fn get_users(state: &TiraState, filter_archived: Option<bool>) -> Result<Vec<User>> {
    dao::users::get_users(state, filter_archived).await
}

/// Service function for updating a user by id.
pub async fn update_user_by_id(state: &TiraState, user: UpdateUser, user_id: i64) -> Result<()> {
    let users_updated = dao::users::update_user_by_id(state, user, user_id).await?;
    service::check_only_one_row_changed(users_updated)
}
