use std::time::SystemTime;

use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::{
    models::{create::CreateTicket, Assignment, Comment, Session, Ticket},
    TiraDbConn,
};

pub async fn create_assignment_by_ticket_id(
    conn: TiraDbConn,
    ticket_id_parameter: i64,
    user_id_parameter: i64,
) {
    use crate::schema::assignments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(assignments)
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
    ticket_id_parameter: i64,
    commenter_id_parameter: i64,
    content_parameter: String,
) {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        diesel::insert_into(comments)
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

pub async fn create_ticket(conn: TiraDbConn, session_uuid: String, ticket: CreateTicket) {
    let the_reporter_id = {
        use crate::schema::sessions::dsl::*;

        let s = conn
            .run(move |c| {
                sessions
                    .filter(uuid.eq(&session_uuid))
                    .first::<Session>(c)
                    .expect("Error loading session.")
            })
            .await;

        if Utc::now().naive_utc() > s.expiration {
            panic!("Session has expired!");
        }

        s.user_id
    };

    {
        use crate::schema::tickets::dsl::*;

        conn.run(move |c| {
            diesel::insert_into(tickets)
                .values((
                    &ticket,
                    created.eq(SystemTime::now()),
                    reporter_id.eq(the_reporter_id),
                ))
                .execute(c)
                .expect("Error with inserting ticket")
        })
        .await;
    }
}

pub async fn delete_ticket_by_id(conn: TiraDbConn, ticket_id: i64) {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        diesel::delete(tickets.filter(id.eq(ticket_id)))
            .execute(c)
            .expect("Failed to delete ticket by id")
    })
    .await;
}

pub async fn delete_tickets(conn: TiraDbConn) {
    use crate::schema::tickets::dsl::*;

    conn.run(|c| {
        diesel::delete(tickets)
            .execute(c)
            .expect("Failed to delete tickets table");
    })
    .await;
}

pub async fn get_assignments_by_ticket_id(
    conn: TiraDbConn,
    ticket_id_parameter: i64,
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

pub async fn get_comments_by_ticket_id(conn: TiraDbConn, ticket_id_parameter: i64) -> Vec<Comment> {
    use crate::schema::comments::dsl::*;

    conn.run(move |c| {
        comments
            .filter(ticket_id.eq(ticket_id_parameter))
            .load::<Comment>(c)
            .expect("Error loading comments.")
    })
    .await
}

pub async fn get_ticket_by_id(conn: TiraDbConn, ticket_id: i64) -> Ticket {
    use crate::schema::tickets::dsl::*;

    conn.run(move |c| {
        tickets
            .filter(id.eq(ticket_id))
            .first::<Ticket>(c)
            .expect("Could not find any ticket.")
    })
    .await
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
