pub mod assignments;
pub mod categories;
pub mod comments;
pub mod sessions;
pub mod tickets;
pub mod users;

// fn get_user_from_session_uuid(conn: TiraDbConn, session_uuid: String) {
//     use crate::schema::sessions::dsl::*;

//     let s = conn
//         .run(move |c| {
//             sessions
//                 .filter(uuid.eq(&session_uuid))
//                 .first::<Session>(c)
//                 .expect("Error loading session.")
//         })
//         .await;

//     if Utc::now().naive_utc() > s.expiration {
//         panic!("Session has expired!");
//     }

//     s.user_id
// }
