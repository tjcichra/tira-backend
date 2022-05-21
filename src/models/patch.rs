use rocket::serde::Deserialize;

use crate::schema::comments;
use crate::schema::users;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct UpdateTicket {
    pub category_id: Option<i64>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub assignee_ids: Option<Vec<i64>>,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "users"]
pub struct UpdateUser {
    pub username: Option<String>,
    pub password: Option<String>,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub profile_picture_url: Option<String>,
    pub archived: Option<bool>,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "comments"]
pub struct UpdateComment {
    pub content: String,
}
