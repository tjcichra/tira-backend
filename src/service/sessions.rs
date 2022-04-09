use uuid::Uuid;

use crate::{TiraDbConn, dao, models::Login, service, controller::TiraMessage};

/// Service function for retrieving user_id by session_uuid.
// pub async fn get_user_id_from_session_uuid(conn: &TiraDbConn, session_uuid: String) -> QueryResult<i64> {
//     let session = dao::sessions::get_session_from_session_uuid(conn, session_uuid).await?;
//     Ok(session.user_id)
// }

/// Service function for performing a login.
/// 
/// Returns the UUID for the newly created session.
pub async fn login(conn: &TiraDbConn, login_info: Login) -> Result<String, TiraMessage> {
    let user = dao::users::get_user_by_username_and_password(conn, login_info).await?;

    let my_uuid = Uuid::new_v4();
    let sessions_created = dao::sessions::create_session_by_session_uuid_and_user_id(conn, my_uuid.to_string(), user.id).await;
    service::check_only_one_row_changed(sessions_created)?;

    Ok(my_uuid.to_string())
}