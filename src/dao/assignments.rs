use crate::{models::Assignment, TiraState};
use anyhow::Result;
use sqlx::QueryBuilder;

/// DAO function for retrieving all assignments.
pub async fn get_assignments(
    state: &TiraState,
    assignee_id: Option<i64>,
    ticket_id: Option<i64>,
) -> Result<Vec<Assignment>> {
    let mut query = QueryBuilder::new(
        "SELECT id, ticket_id, assignee_id, assigner_id, assigned from assignments where 1=1 ",
    );

    if let Some(assignee_id) = assignee_id {
        query.push("and assignee_id = ");
        query.push_bind(assignee_id);
    }

    if let Some(ticket_id) = ticket_id {
        query.push("and ticket_id = ");
        query.push_bind(ticket_id);
    }

    let assignments = query
        .build_query_as::<Assignment>()
        .fetch_all(&state.pool)
        .await?;

    Ok(assignments)
}

/// DAO function for updating assignments by ticket id.
pub async fn update_assignments_by_ticket_id(
    state: &TiraState,
    ticket_id: i64,
    assignee_ids: Vec<i64>,
    assigner_id: i64,
) -> Result<()> {
    //  TODO: use transaction
    for id in assignee_ids {
        sqlx::query!(
            "UPDATE assignments SET assignee_id = $1, assigner_id = $2 WHERE ticket_id = $3 RETURNING ticket_id",id, assigner_id, ticket_id
        ).fetch_all(&state.pool).await?;
    }

    Ok(())
}
