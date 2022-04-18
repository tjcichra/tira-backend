use rocket::serde::{Deserialize};

use crate::schema::tickets;
use crate::schema::users;

#[derive(AsChangeset, Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
#[table_name = "tickets"]
pub struct UpdateTicket {
    pub category_id: Option<i64>,
    pub subject: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
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