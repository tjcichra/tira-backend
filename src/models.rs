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
