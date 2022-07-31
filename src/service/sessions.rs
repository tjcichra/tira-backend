use uuid::Uuid;

use crate::{
    controller::{self, TiraErrorResponse},
    dao,
    models::{Login, User},
    service, TiraDbConn,
};

/// Service function for retrieving user_id by session_uuid.
// pub async fn get_user_id_from_session_uuid(conn: &TiraDbConn, session_uuid: String) -> QueryResult<i64> {
//     let session = dao::sessions::get_session_from_session_uuid(conn, session_uuid).await?;
//     Ok(session.user_id)
// }

/// Service function for performing a login.
///
/// Returns the UUID for the newly created session and user.
pub async fn login(
    conn: &TiraDbConn,
    login_info: Login,
) -> Result<(String, User), TiraErrorResponse> {
    let remember_me = login_info.remember_me;
    let user = dao::users::get_user_by_username_and_password(conn, login_info)
        .await
        .map_err(controller::convert)?;

    let my_uuid = Uuid::new_v4();
    dao::sessions::create_session_by_session_uuid_and_user_id(
        conn,
        my_uuid.to_string(),
        user.id,
        remember_me,
    )
    .await
    .map_err(controller::convert)?;

    Ok((my_uuid.to_string(), user))
}

/// Service function for having a user log out.
pub async fn logout(conn: &TiraDbConn, user_id: i64) -> Result<(), TiraErrorResponse> {
    let sessions_deleted = dao::sessions::delete_sessions_by_user_id(conn, user_id)
        .await
        .map_err(controller::convert)?;
    service::check_at_least_one_row_changed(sessions_deleted)
}
