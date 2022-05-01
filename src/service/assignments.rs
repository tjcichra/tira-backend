use crate::{dao, TiraDbConn, models::Assignment, controller::{TiraErrorResponse, self}};

/// Service function for retrieving all assignments.
pub async fn get_assignments(
    conn: &TiraDbConn,
    assignee_id: Option<i64>,
) -> Result<Vec<Assignment>, TiraErrorResponse> {
    dao::assignments::get_assignments(conn, assignee_id).await.map_err(controller::convert)
}