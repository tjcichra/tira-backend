use crate::{dao, models::Assignment, TiraState};
use anyhow::Result;

/// Service function for retrieving all assignments.
pub async fn get_assignments(
    state: &TiraState,
    assignee_id: Option<i64>,
    ticket_id: Option<i64>,
) -> Result<Vec<Assignment>> {
    let assignments = dao::assignments::get_assignments(state, assignee_id, ticket_id).await?;
    Ok(assignments)
}
