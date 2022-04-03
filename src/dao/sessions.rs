use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, QueryResult};

use crate::{TiraDbConn, models::Session};
use std::time::SystemTime;
use std::time::Duration;

/// DAO function for retrieving session by session_uuid.
pub async fn get_session_from_session_uuid(conn: &TiraDbConn, session_uuid: String) -> QueryResult<Session> {
    use crate::schema::sessions::dsl::*;

    conn
        .run(move |c| {
            sessions
                .filter(uuid.eq(&session_uuid))
                .first::<Session>(c)
        })
        .await
}

/// DAO function for creating session by session_uuid and user_id.
pub async fn create_session_by_session_uuid_and_user_id(conn: &TiraDbConn, session_uuid: String, user_id_parameter: i64) -> QueryResult<usize> {
    use crate::schema::sessions::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(sessions)
            .values((
                uuid.eq(session_uuid),
                user_id.eq(user_id_parameter),
                created.eq(Utc::now().naive_utc()),
                expiration.eq(SystemTime::now() + Duration::from_secs(30)), //TODO: Fix this
            ))
            .execute(c)
    })
    .await
}