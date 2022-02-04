use std::time::SystemTime;

use diesel::{delete, insert_into, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{Assignment, Comment, CreateTicket, Ticket},
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

pub async fn create_assignment_by_ticket_id(
    conn: TiraDbConn,
    ticket_id_parameter: i32,
    user_id_parameter: i32,
) {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        insert_into(assignments)
            .values((
                ticket_id.eq(ticket_id_parameter),
                user_id.eq(user_id_parameter),
                assigned.eq(SystemTime::now()),
            ))
            .execute(c)
            .expect("Error with inserting assignment")
    })
    .await;
}

pub async fn create_comment(
    conn: TiraDbConn,
    ticket_id_parameter: i32,
    commenter_id_parameter: i32,
    content_parameter: String,
) {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        insert_into(comments)
            .values((
                ticket_id.eq(ticket_id_parameter),
                commenter_id.eq(commenter_id_parameter),
                content.eq(content_parameter),
                commented.eq(SystemTime::now()),
            ))
            .execute(c)
            .expect("Error with inserting comment")
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

pub async fn get_assignments_by_ticket_id(
    conn: TiraDbConn,
    ticket_id_parameter: i32,
) -> Vec<Assignment> {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        assignments
            .filter(ticket_id.eq(ticket_id_parameter))
            .load::<Assignment>(c)
            .expect("Could not find any assignments.")
    })
    .await
}

pub async fn get_comments_by_ticket_id(conn: TiraDbConn, ticket_id_parameter: i32) -> Vec<Comment> {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        comments
            .filter(ticket_id.eq(ticket_id_parameter))
            .load::<Comment>(c)
            .expect("Error loading comments.")
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
