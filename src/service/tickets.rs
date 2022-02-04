use std::time::SystemTime;

use diesel::{delete, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{CreateTicket, Ticket},
    TiraDbConn,
};

pub async fn create_ticket(conn: TiraDbConn, ticket: CreateTicket) {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        insert_into(tickets)
            .values((&ticket, created.eq(SystemTime::now())))
            .execute(c)
            .expect("Error with inserting ticket")
    })
    .await;
}

pub async fn get_tickets(conn: TiraDbConn) -> Vec<Ticket> {
    use crate::schema::tickets::dsl::*;

    conn.run(|c| {
        tickets
            .load::<Ticket>(c)
            .expect("Error with retrieving tickets")
    })
    .await
}

pub async fn get_ticket_by_id(conn: TiraDbConn, ticket_id: i32) -> Ticket {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        tickets
            .filter(id.eq(ticket_id))
            .first::<Ticket>(c)
            .expect("Could not find any ticket.")
    })
    .await
}

pub async fn delete_tickets(conn: TiraDbConn) {
    use crate::schema::tickets::dsl::*;

    conn.run(|c| {
        delete(tickets)
            .execute(c)
            .expect("Failed to delete tickets table");
    })
    .await;
}

pub async fn delete_ticket_by_id(conn: TiraDbConn, ticket_id: i32) {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        delete(tickets.filter(id.eq(ticket_id)))
            .execute(c)
            .expect("Failed to delete ticket by id")
    })
    .await;
}
