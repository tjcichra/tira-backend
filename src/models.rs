use crate::schema::assignments;
use crate::schema::categories;
use crate::schema::comments;
use crate::schema::sessions;
use crate::schema::tickets;
use chrono::NaiveDateTime;
use rocket::serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    #[serde(default)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "categories"]
#[serde(crate = "rocket::serde")]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Ticket {
    pub id: i32,
    pub category_id: Option<i32>,
    pub subject: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub created: NaiveDateTime,
    pub reporter_id: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "tickets"]
#[serde(crate = "rocket::serde")]
pub struct CreateTicket {
    pub category_id: Option<i32>,
    pub subject: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Assignment {
    pub ticket_id: i32,
    pub user_id: i32,
    pub assigned: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "assignments"]
#[serde(crate = "rocket::serde")]
pub struct CreateAssignmentWithUserId {
    pub user_id: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Comment {
    pub id: i32,
    pub ticket_id: i32,
    pub commenter_id: i32,
    pub content: String,
    pub commented: NaiveDateTime,
}

#[derive(Deserialize, Insertable)]
#[table_name = "comments"]
#[serde(crate = "rocket::serde")]
pub struct CreateComment {
    pub commenter_id: i32,
    pub content: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Session {
    pub uuid: String,
    pub user_id: i32,
    pub created: NaiveDateTime,
    pub expiration: NaiveDateTime,
}
