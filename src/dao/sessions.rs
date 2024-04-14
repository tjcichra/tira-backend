use chrono::{Duration, Utc};

use crate::TiraState;
use std::env;

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
    state: &TiraState,
    session_uuid: String,
    user_id: i64,
    remember_me: bool,
) -> anyhow::Result<String> {
    if remember_me {
        let result = sqlx::query!(
            "INSERT INTO sessions (uuid, user_id) VALUES ($1, $2) RETURNING uuid",
            session_uuid,
            user_id
        )
        .fetch_one(&state.pool)
        .await?;
        Ok(result.uuid)
    } else {
        let session_length_minutes_env: i64 =
            env::var("SESSION_LENGTH_MINUTES").unwrap().parse().unwrap();

        let expires = (Utc::now() + Duration::minutes(session_length_minutes_env)).naive_utc();

        let result = sqlx::query!(
            "INSERT INTO sessions (uuid, user_id, expiration) VALUES ($1, $2, $3) RETURNING uuid",
            session_uuid,
            user_id,
            expires,
        )
        .fetch_one(&state.pool)
        .await?;

        Ok(result.uuid)
    }
}

/// DAO function for deleting sessions by user id and uuid.
pub async fn delete_sessions_by_user_id_and_uuid(
    state: &TiraState,
    user_id: i64,
    uuid: String,
) -> anyhow::Result<u64> {
    let result = sqlx::query!(
        "DELETE FROM sessions WHERE user_id = $1 AND uuid = $2",
        user_id,
        uuid
    )
    .execute(&state.pool)
    .await?;
    Ok(result.rows_affected())
}
