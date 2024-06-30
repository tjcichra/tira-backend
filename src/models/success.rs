use super::TicketWithReporterAsUser;
use crate::models::{Category, User};
use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Serialize)]
pub struct StandardResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct AlteredResourceResponse {
    pub message: String,
    pub id: i64,
}

#[derive(Serialize)]
pub struct EditTicketResponse {
    pub id: i64,
}

#[derive(Serialize)]
pub struct CommentResponse {
    pub id: i64,
    pub commenter: User,
    pub content: String,
    pub commented: NaiveDateTime,
}

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: i64,
    pub subject: String,
    pub description: Option<String>,
    pub category: Option<Category>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter: User,
    pub assignees: Vec<User>,
}

#[derive(Serialize)]
pub struct CountResponse<T> {
    pub data: Vec<T>,
    pub total_count: i64,
}

#[derive(Serialize)]
pub struct TicketWithoutDescriptionResponse {
    pub id: i64,
    pub subject: String,
    pub category: Option<Category>,
    pub priority: String,
    pub status: String,
    pub created: NaiveDateTime,
    pub reporter: User,
    pub assignees: Vec<User>,
}

#[derive(Serialize)]
pub struct AssignmentResponse {
    pub id: i64,
    pub ticket: TicketWithReporterAsUser,
    pub assigner: User,
    pub assigned: NaiveDateTime,
}
