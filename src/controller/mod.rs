pub mod categories;
pub mod users;

use rocket::response::content::Custom;
use rocket::{
    http::ContentType,
    serde::{json::Json, Serialize},
    Request,
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    detail: String,
    #[serde(rename = "type")]
    type_: String,
    status: i32,
}

#[catch(404)]
pub fn not_found(req: &Request) -> Custom<Json<ErrorResponse>> {
    let custom = ContentType::new("application", "problem+json");

    Custom(
        custom,
        Json(ErrorResponse {
            detail: format!("Sorry, '{}' is not a valid path.", req.uri()),
            type_: "https://docs.diesel.rs/diesel/result/enum.Error.html".to_string(),
            status: 404,
        }),
    )
}
