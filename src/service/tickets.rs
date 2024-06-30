use crate::{
    dao::{self, tickets},
    models::{
        patch::UpdateTicket, Assignment, Comment, CreateTicket, Ticket, TicketWithoutDescription,
    },
    service, TiraState,
};
use anyhow::anyhow;
use anyhow::Result;
use regex::Regex;

/// Service function for creating an assignment by ticket id and assigner id.
pub async fn create_assignment_by_ticket_id_and_assigner_id(
    state: &TiraState,
    assignee_id: i64,
    ticket_id: i64,
    assigner_id: i64,
) -> Result<i64> {
    dao::tickets::create_assignment_by_ticket_id_and_assigner_id(
        state,
        assignee_id,
        ticket_id,
        assigner_id,
    )
    .await
}

/// Service function for creating a comment by ticket id.
pub async fn create_comment_by_ticket_id_and_commenter_id(
    state: &TiraState,
    comment: &str,
    ticket_id: i64,
    commenter_id: i64,
) -> Result<i64> {
    let regex = Regex::new(r"(</?[^>]+(>|$)|&nbsp;|\s)").unwrap();
    let content_without_tags: String = regex.replace_all(comment, "").into();

    if content_without_tags.is_empty() {
        return Err(anyhow!("Comment cannot be blank!"));
    }

    dao::tickets::create_comment_by_ticket_id_and_commenter_id(
        state,
        comment,
        ticket_id,
        commenter_id,
    )
    .await
}

/// Service function for creating a ticket by reporter id.
pub async fn create_ticket_by_reporter_id(
    state: &TiraState,
    ticket: &CreateTicket,
    reporter_id: i64,
) -> Result<i64> {
    if ticket.subject.is_empty() {
        return Err(anyhow!("Subject can not be empty"));
    }

    match ticket.status.as_str() {
        "Backlog" | "In Progress" | "Not Deployed Yet" | "Done" | "Closed" => (),
        _ => {
            return Err(anyhow!(
                "Status must be 'Backlog', 'In Progress', 'Not Deployed Yet', 'Done', or 'Closed'"
            ));
        }
    }

    match ticket.priority.as_str() {
        "Low" | "Medium" | "High" => (),
        _ => {
            return Err(anyhow!("Priority must be 'Low', 'Medium', or 'High'"));
        }
    }

    let id = dao::tickets::create_ticket_by_reporter_id(state, ticket, reporter_id).await?;
    Ok(id)
}

/// Service function for retrieving assignments by ticket id.
pub async fn get_assignments_by_ticket_id(
    state: &TiraState,
    ticket_id: i64,
) -> Result<Vec<Assignment>> {
    let assignments = dao::tickets::get_assignments_by_ticket_id(state, ticket_id).await?;
    Ok(assignments)
}

/// Service function for retrieving comments by ticket id.
pub async fn get_comments_by_ticket_id(state: &TiraState, ticket_id: i64) -> Result<Vec<Comment>> {
    let comments = dao::tickets::get_comments_by_ticket_id(state, ticket_id).await?;
    Ok(comments)
}

/// Service function for retrieving a ticket by id.
pub async fn get_ticket_by_id(state: &TiraState, ticket_id: i64) -> Result<Ticket> {
    let ticket = dao::tickets::get_ticket_by_id(state, ticket_id).await?;
    Ok(ticket)
}

/// Service function for retrieving tickets by ids.
pub async fn get_tickets_by_ids(state: &TiraState, ticket_ids: Vec<i64>) -> Result<Vec<Ticket>> {
    let tickets = dao::tickets::get_tickets_by_ids(state, ticket_ids).await?;
    Ok(tickets)
}

/// Service function for retrieving all tickets.
pub async fn get_tickets(
    state: &TiraState,
    // limit: Option<i64>,
    // offset: Option<i64>,
    // filter_reporter_id: Option<i64>,
    // filter_open: Option<bool>,
    // sort_by: Option<String>,
    // order_by: Option<String>,
) -> Result<(Vec<TicketWithoutDescription>, i64)> {
    let tickets = dao::tickets::get_tickets(
        state,
        // limit,
        // offset,
        // filter_reporter_id,
        // filter_open,
        // sort_by,
        // order_by,
    )
    .await?;

    let total_count = dao::tickets::get_total_ticket_count(state).await?;
    Ok((tickets, total_count))
}

/// Service function for updating a ticket by id.
pub async fn update_ticket_by_id(
    state: &TiraState,
    ticket: &UpdateTicket,
    ticket_id: i64,
) -> Result<()> {
    let tickets_updated = tickets::update_ticket_by_id(state, ticket, ticket_id).await?;
    service::check_only_one_row_changed(tickets_updated)
}

/// Service function for updating assignments by ticket id.
pub async fn update_assignments_by_ticket_id(
    state: &TiraState,
    ticket_id: i64,
    assignee_ids: Vec<i64>,
    assigner_id: i64,
) -> Result<()> {
    dao::assignments::update_assignments_by_ticket_id(state, ticket_id, assignee_ids, assigner_id)
        .await
}
