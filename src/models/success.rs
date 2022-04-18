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
