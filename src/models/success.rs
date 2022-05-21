use crate::models::{Category, User};
use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct StandardResponse {
    pub message: String,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AlteredResourceResponse {
    pub message: String,
    pub id: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct EditTicketResponse {
    pub id: i64,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct CommentResponse {
    pub id: i64,
    pub commenter: User,
    pub content: String,
    pub commented: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TicketResponse {
    pub id: i64,
    pub subject: String,
    pub description: Option<String>,
    pub category_id: Option<i64>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter: User,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TicketWithoutDescriptionResponse {
    pub id: i64,
    pub subject: String,
    pub category: Option<Category>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter: User,
}
