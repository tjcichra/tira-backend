use chrono::Utc;
use diesel::QueryDsl;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};

use crate::TiraDbConn;
use std::env;
use std::time::Duration;
use std::time::SystemTime;

/// DAO function for retrieving session by session_uuid.
// pub async fn get_session_from_session_uuid(conn: &TiraDbConn, session_uuid: String) -> QueryResult<Session> {
//     use crate::schema::sessions::dsl::*;

//     conn
//         .run(move |c| {
//             sessions
//                 .filter(uuid.eq(&session_uuid))
//                 .first::<Session>(c)
//         })
//         .await
// }

/// DAO function for creating session by session_uuid and user_id.
pub async fn create_session_by_session_uuid_and_user_id(
    conn: &TiraDbConn,
    session_uuid: String,
    user_id_parameter: i64,
    remember_me: bool,
) -> QueryResult<String> {
    use crate::schema::sessions::dsl::*;

    let expiration_parameter = if remember_me {
        None
    } else {
        let session_length_minutes_env: u64 =
            env::var("SESSION_LENGTH_MINUTES").unwrap().parse().unwrap();
        Some(
            expiration.eq(SystemTime::now() + Duration::from_secs(session_length_minutes_env * 60)),
        ) //TODO: Fix this
    };

    conn.run(move |c| {
        diesel::insert_into(sessions)
            .values((
                uuid.eq(session_uuid),
                user_id.eq(user_id_parameter),
                created.eq(Utc::now().naive_utc()),
                expiration_parameter,
            ))
            .returning(uuid)
            .get_result(c)
    })
    .await
}

/// DAO function for deleting sessions by user_id.
pub async fn delete_sessions_by_user_id(
    conn: &TiraDbConn,
    user_id_parameter: i64,
) -> QueryResult<usize> {
    use crate::schema::sessions::dsl::*;

    conn.run(move |c| diesel::delete(sessions.filter(user_id.eq(user_id_parameter))).execute(c))
        .await
}
