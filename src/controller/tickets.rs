use crate::models::{
    create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
    Assignment, Comment, Ticket,
};
use crate::service::tickets;
use crate::TiraDbConn;
use rocket::http::CookieJar;
use rocket::serde::json::Json;

#[post("/tickets/<ticket_id>/assignments", data = "<create_assignment_json>")]
pub async fn create_assignment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
    create_assignment_json: Json<CreateAssignmentWithUserId>,
) {
    tickets::create_assignment_by_ticket_id(conn, ticket_id, create_assignment_json.0.user_id)
        .await;
}

#[post("/tickets/<ticket_id>/comments", data = "<create_comment_json>")]
pub async fn create_comment_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
    create_comment_json: Json<CreateComment>,
) {
    tickets::create_comment(
        conn,
        ticket_id,
        create_comment_json.0.commenter_id,
        create_comment_json.0.content,
    )
    .await;
}

#[post("/tickets", data = "<create_ticket_json>")]
pub async fn create_ticket_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_ticket_json: Json<CreateTicket>,
) {
    tickets::create_ticket(
        conn,
        cookies.get("tirauth").unwrap().value().to_owned(),
        create_ticket_json.0,
    )
    .await;
}

#[delete("/tickets/<ticket_id>")]
pub async fn delete_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i64) {
    tickets::delete_ticket_by_id(conn, ticket_id).await;
}

#[delete("/tickets")]
pub async fn delete_tickets_endpoint(conn: TiraDbConn) {
    tickets::delete_tickets(conn).await;
}

#[get("/tickets/<ticket_id>/assignments")]
pub async fn get_assignments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> Json<Vec<Assignment>> {
    Json(tickets::get_assignments_by_ticket_id(conn, ticket_id).await)
}

#[get("/tickets/<ticket_id>/comments")]
pub async fn get_comments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> Json<Vec<Comment>> {
    Json(tickets::get_comments_by_ticket_id(conn, ticket_id).await)
}

#[get("/tickets/<ticket_id>")]
pub async fn get_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i64) -> Json<Ticket> {
    Json(tickets::get_ticket_by_id(conn, ticket_id).await)
}

#[get("/tickets")]
pub async fn get_tickets_endpoint(conn: TiraDbConn) -> Json<Vec<Ticket>> {
    Json(tickets::get_tickets(conn).await)
}
