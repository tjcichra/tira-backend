use std::time::{Duration, SystemTime};

use crypto::{digest::Digest, sha2::Sha256};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use uuid::Uuid;

use crate::{models::User, TiraDbConn};

pub mod categories;
pub mod tickets;
pub mod users;

pub async fn login(
    conn: TiraDbConn,
    username_input: String,
    password_input: String,
) -> QueryResult<String> {
    let user = {
        use crate::schema::users::dsl::*;

        let mut hasher = Sha256::new();
        hasher.input_str(&password_input);
        let hex = hasher.result_str();

        conn.run(move |c| {
            users
                .filter(username.eq(username_input))
                .filter(password.eq(hex))
                .first::<User>(c)
        })
        .await?
    };

    use crate::schema::sessions::dsl::*;

    let my_uuid = Uuid::new_v4();

    conn.run(move |c| {
        diesel::insert_into(sessions)
            .values((
                uuid.eq(my_uuid.to_string()),
                user_id.eq(user.id),
                created.eq(SystemTime::now()),
                expiration.eq(SystemTime::now() + Duration::from_secs(30)),
            ))
            .execute(c)
    })
    .await?;

    Ok(my_uuid.to_string())
}
