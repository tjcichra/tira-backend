#[derive(Queryable, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email_address: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>
}