use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub mod patch;
pub mod success;

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub creator_id: i64,
    pub created: NaiveDateTime,
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateComment {
    pub content: String,
}
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateTicket {
    pub category_id: Option<i64>,
    pub subject: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub assignee_ids: Vec<i64>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
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

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct CreateAssignmentWithUserId {
    pub assignee_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TicketWithReporterAsUser {
    pub id: i64,
    pub subject: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter: User,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct TicketWithoutDescription {
    pub id: i64,
    pub subject: String,
    pub category_id: Option<i64>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter_id: i64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Assignment {
    pub id: i64,
    pub ticket_id: i64,
    pub assignee_id: i64,
    pub assigner_id: i64,
    pub assigned: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: i64,
    pub ticket_id: i64,
    pub commenter_id: i64,
    pub content: String,
    pub commented: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Session {
    pub uuid: String,
    pub user_id: i64,
    pub created: NaiveDateTime,
    pub expiration: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Login {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Count {
    pub cnt: Option<i64>,
}

#[derive(sqlx::FromRow)]
pub struct ReturningId {
    pub id: i64,
}
