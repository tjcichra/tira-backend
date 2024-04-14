use crate::{
    dao,
    models::{Login, User},
    service, TiraState,
};
use anyhow::Result;
use uuid::Uuid;

/// Service function for retrieving user_id by session_uuid.
// pub async fn get_user_id_from_session_uuid(conn: &TiraDbConn, session_uuid: String) -> QueryResult<i64> {
//     let session = dao::sessions::get_session_from_session_uuid(conn, session_uuid).await?;
//     Ok(session.user_id)
// }

/// Service function for performing a login.
///
/// Returns the UUID for the newly created session and user.
pub async fn login(state: &TiraState, login_info: Login) -> Result<(String, User)> {
    let remember_me = login_info.remember_me;
    let user = dao::users::get_user_by_username_and_password(state, login_info).await?;

    let my_uuid = Uuid::new_v4();
    dao::sessions::create_session_by_session_uuid_and_user_id(
        state,
        my_uuid.to_string(),
        user.id,
        remember_me,
    )
    .await?;

    Ok((my_uuid.to_string(), user))
}

/// Service function for having a user log out.
pub async fn logout(state: &TiraState, user_id: i64, session_uuid: String) -> Result<()> {
    let sessions_deleted =
        dao::sessions::delete_sessions_by_user_id_and_uuid(state, user_id, session_uuid).await?;
    service::check_only_one_row_changed(sessions_deleted)
}
