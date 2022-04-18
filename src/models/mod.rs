use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

pub mod patch;
pub mod success;

pub mod create {
    use crate::schema::assignments;
    use crate::schema::categories;
    use crate::schema::comments;
    use crate::schema::tickets;
    use crate::schema::users;
    use rocket::serde::Deserialize;

    use super::Assignment;

    #[derive(Deserialize, Insertable)]
    #[table_name = "assignments"]
    #[serde(crate = "rocket::serde")]
    pub struct CreateAssignmentWithUserId {
        pub assignee_id: i64,
    }

    #[derive(Deserialize, Insertable)]
    #[table_name = "categories"]
    #[serde(crate = "rocket::serde")]
    pub struct CreateCategory {
        pub name: String,
        pub description: Option<String>,
    }

    #[derive(Deserialize, Insertable)]
    #[table_name = "comments"]
    #[serde(crate = "rocket::serde")]
    pub struct CreateComment {
        pub content: String,
    }

    #[derive(Deserialize)]
    #[serde(crate = "rocket::serde")]
    pub struct CreateTicket {
        pub category_id: Option<i64>,
        pub subject: String,
        pub description: Option<String>,
        pub status: String,
        pub priority: String,
        pub assignee_ids: Vec<i64>,
    }

    #[derive(Deserialize, Insertable)]
    #[table_name = "users"]
    #[serde(crate = "rocket::serde")]
    pub struct CreateUser {
        pub username: String,
        pub password: String,
        pub email_address: Option<String>,
        pub first_name: Option<String>,
        pub last_name: Option<String>,
        pub profile_picture_url: Option<String>,
    }
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub created: NaiveDateTime,
    pub archived: bool,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: i64,
    pub created: NaiveDateTime,
    pub archived: bool,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    pub id: i64,
    pub subject: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter_id: i64,
}

#[derive(Queryable, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TicketWithoutDescription {
    pub id: i64,
    pub subject: String,
    #[serde(skip_serializing)]
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter_id: i64,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Assignment {
    pub id: i64,
    pub ticket_id: i64,
    pub assignee_id: i64,
    pub assigner_id: i64,
    pub assigned: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: i64,
    pub ticket_id: i64,
    pub commenter_id: i64,
    pub content: String,
    pub commented: NaiveDateTime,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub uuid: String,
    pub user_id: i64,
    pub created: NaiveDateTime,
    pub expiration: Option<NaiveDateTime>,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}
