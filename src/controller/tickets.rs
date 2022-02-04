use crate::models::{CreateAssignmentWithUserId, CreateTicket, Ticket, Assignment};
use crate::service::tickets::{
    create_ticket, delete_ticket_by_id, delete_tickets,
    get_ticket_by_id, get_tickets, create_assignment_by_ticket_id, get_assignments_by_ticket_id,
};
use crate::TiraDbConn;
use rocket::serde::json::Json;

#[post("/tickets", data = "<create_ticket_json>")]
pub async fn create_ticket_endpoint(conn: TiraDbConn, create_ticket_json: Json<CreateTicket>) {
    create_ticket(conn, create_ticket_json.0).await;
}

#[post("/tickets/<ticket_id>/assignments", data = "<create_assignment_json>")]
pub async fn create_assignment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i32,
    create_assignment_json: Json<CreateAssignmentWithUserId>,
) {
    create_assignment_by_ticket_id(conn, ticket_id, create_assignment_json.0.user_id).await;
}

#[get("/tickets/<ticket_id>/assignments")]
pub async fn get_assignments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i32,
) -> Json<Vec<Assignment>> {
    Json(get_assignments_by_ticket_id(conn, ticket_id).await)
}

#[get("/tickets")]
pub async fn get_tickets_endpoint(conn: TiraDbConn) -> Json<Vec<Ticket>> {
    Json(get_tickets(conn).await)
}

#[get("/tickets/<ticket_id>")]
pub async fn get_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i32) -> Json<Ticket> {
    Json(get_ticket_by_id(conn, ticket_id).await)
}

#[delete("/tickets")]
pub async fn delete_tickets_endpoint(conn: TiraDbConn) {
    delete_tickets(conn).await;
}

#[delete("/tickets/<ticket_id>")]
pub async fn delete_ticket_by_id_endpoint(conn: TiraDbConn, ticket_id: i32) {
    delete_ticket_by_id(conn, ticket_id).await;
}
