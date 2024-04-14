use crate::{service, TiraState};
use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;

use super::TiraError;

#[derive(Deserialize)]
pub struct AssignmentQueryParams {
    assignee_id: Option<i64>,
    ticket_id: Option<i64>,
}

/// Endpoint for retrieving every assignment.
///
/// **GET /assignments**
///
/// Query Parameters:
///
/// assignee_id: Used to filter assignments that were assigned to a certain user. Takes a number value. (optional)
/// ticket_id: Used to filter assignments that a certain ticket has. Takes a number value. (optional)
pub async fn get_assignments_endpoint(
    State(state): State<TiraState>,
    query_params: Query<AssignmentQueryParams>,
) -> Result<Response, TiraError> {
    let assignments = service::assignments::get_assignments(
        &state,
        query_params.assignee_id,
        query_params.ticket_id,
    )
    .await?;
    Ok(Json(assignments).into_response())
}
