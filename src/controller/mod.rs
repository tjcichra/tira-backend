pub mod categories;
pub mod tickets;
pub mod users;

use diesel::QueryResult;
use rocket::http::{Cookie, CookieJar};
use rocket::response::content::Custom;
use rocket::{
    http::ContentType,
    serde::{json::Json, Deserialize, Serialize},
    Request,
};

use crate::{service, TiraDbConn};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct ErrorResponse {
    detail: String,
    #[serde(rename = "type")]
    type_: String,
    status: i32,
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<login_info>")]
pub async fn login_endpoint(conn: TiraDbConn, cookies: &CookieJar<'_>, login_info: Json<Login>) {
    let login_info = login_info.0;
    let uuid = service::login(conn, login_info.username, login_info.password)
        .await
        .unwrap();
    cookies.add(Cookie::new("tirauth", uuid));
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

pub type TiraResponse<T> = Result<Json<T>, Json<String>>;

fn standardize_response<T>(result: QueryResult<T>) -> TiraResponse<T> {
    result
        .map(|value| Json(value))
        .map_err(|err| Json(err.to_string()))
}
