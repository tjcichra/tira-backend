use rocket::serde::{Deserialize};

use crate::schema::tickets;

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