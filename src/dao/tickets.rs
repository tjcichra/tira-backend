use crate::{
    models::{
        patch::UpdateTicket, Assignment, Comment, Count, CreateTicket, ReturningId, Ticket,
        TicketWithoutDescription,
    },
    TiraState,
};
use anyhow::Result;
use sqlx::QueryBuilder;

/// DAO function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    state: &TiraState,
    assignee_id: i64,
    ticket_id: i64,
    assigner_id: i64,
) -> Result<i64> {
    let result =  sqlx::query!("INSERT INTO assignments (assignee_id, ticket_id, assigner_id, assigned) VALUES ($1, $2, $3, NOW()) RETURNING id",assignee_id,ticket_id, assigner_id)
    .fetch_all(&state.pool).await?;

    let id = result.first().unwrap().id;

    Ok(id)
}

/// DAO function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    state: &TiraState,
    content: &str,
    ticket_id: i64,
    commenter_id: i64,
) -> Result<i64> {
    let result = sqlx::query!(
        "INSERT INTO comments (ticket_id, commenter_id, content) VALUES ($1, $2, $3) RETURNING id",
        ticket_id,
        commenter_id,
        content,
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(result.id)
}

/// DAO function for creating a ticket by reporter id and assigning those tickets.
///
/// Returns the id of the new ticket.
pub async fn create_ticket_by_reporter_id(
    state: &TiraState,
    ticket: &CreateTicket,
    reporter_id: i64,
) -> Result<i64> {
    // TODO: make a sql transaction
    let result =  sqlx::query!(
        "INSERT INTO tickets (category_id, subject, description, status, priority, reporter_id) VALUES ($1,$2,$3,$4,$5,$6) RETURNING id", 
        ticket.category_id,
        ticket.subject.clone(),
        ticket.description.clone(),
        ticket.status.clone(),
        ticket.priority.clone(),
        reporter_id,
    )
     .fetch_one(&state.pool).await?;

    let ticket_id = result.id;

    for assignee in &ticket.assignee_ids {
        sqlx::query!(
            "INSERT INTO assignments (ticket_id, assignee_id, assigner_id) VALUES ($1,$2,$3)",
            ticket_id,
            assignee,
            reporter_id
        )
        .fetch_one(&state.pool)
        .await?;
    }

    // tx.commit().await?;

    Ok(result.id)
}

/// DAO function for retrieving assignments by ticket id.
pub async fn get_assignments_by_ticket_id(
    state: &TiraState,
    ticket_id: i64,
) -> Result<Vec<Assignment>> {
    let assignments = sqlx::query_as!(
        Assignment,
        "SELECT * FROM assignments WHERE ticket_id = $1",
        ticket_id
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(assignments)
}

/// DAO function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(state: &TiraState, ticket_id: i64) -> Result<Vec<Comment>> {
    let comments = sqlx::query_as!(
        Comment,
        "SELECT * FROM comments WHERE ticket_id = $1",
        ticket_id
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(comments)
}

/// DAO function for retrieving a ticket by id.
pub async fn get_ticket_by_id(state: &TiraState, ticket_id: i64) -> Result<Ticket> {
    let ticket = sqlx::query_as!(Ticket, "SELECT * FROM tickets WHERE id = $1", ticket_id)
        .fetch_one(&state.pool)
        .await?;
    Ok(ticket)
}

/// DAO function for retrieving tickets by ids.
pub async fn get_tickets_by_ids(state: &TiraState, ticket_ids: Vec<i64>) -> Result<Vec<Ticket>> {
    let tickets = sqlx::query_as!(
        Ticket,
        "SELECT * FROM tickets WHERE id in (SELECT unnest($1::integer[]))",
        &ticket_ids
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(tickets)
}

/// DAO function for retrieving all tickets.
pub async fn get_tickets(
    state: &TiraState,
    limit: Option<i64>,
    offset: Option<i64>,
    filter_reporter_id: Option<i64>,
    filter_open: Option<bool>,
    sort_by: Option<String>,
    order_by: Option<String>,
) -> Result<Vec<TicketWithoutDescription>> {
    // TODO use query builder - right now ignore all the filter options

    let tickets = sqlx::query_as!(TicketWithoutDescription, "SELECT * FROM tickets")
        .fetch_all(&state.pool)
        .await?;
    Ok(tickets)
}

/// DAO function for retrieving the total ticket count.
pub async fn get_total_ticket_count(state: &TiraState) -> Result<i64> {
    let result = sqlx::query_as!(Count, "SELECT count(*) cnt FROM tickets")
        .fetch_one(&state.pool)
        .await?;
    Ok(result.cnt.unwrap_or(0))
}

/// DAO function for updating a ticket by id.
pub async fn update_ticket_by_id(
    state: &TiraState,
    ticket: &UpdateTicket,
    ticket_id: i64,
) -> Result<u64> {
    let mut query = QueryBuilder::new("UPDATE TICKETS SET ");
    if let Some(category_id) = ticket.category_id {
        query.push("category_id = ");
        query.push_bind(category_id);
    }
    if let Some(subject) = ticket.subject.clone() {
        query.push("subject = ");
        query.push_bind(subject);
    }
    if let Some(description) = ticket.description.clone() {
        query.push("description = ");
        query.push_bind(description);
    }
    if let Some(status) = ticket.status.clone() {
        query.push("status = ");
        query.push_bind(status);
    }
    if let Some(priority) = ticket.priority.clone() {
        query.push("priority = ");
        query.push_bind(priority);
    }
    if let Some(assignee_ids) = ticket.assignee_ids.clone() {
        query.push("assignee_ids = ");
        query.push_bind(assignee_ids);
    }

    query.push(" WHERE id = ");
    query.push_bind(ticket_id);

    let result = query.build().execute(&state.pool).await?;

    Ok(result.rows_affected())
}
