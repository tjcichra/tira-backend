use crate::{TiraDbConn, controller::{self, TiraResponse}, models::Assignment, service};

/// Endpoint for retrieving every assignment.
///
/// **GET /assignments**
///
/// Query Parameters:
///
/// assignee_id: Used to filter assignments that were assigned to a certain user. Takes a number value. (optional)
#[get("/assignments?<assignee_id>")]
pub async fn get_assignments_endpoint(
    conn: TiraDbConn,
    assignee_id: Option<i64>,
) -> TiraResponse<Vec<Assignment>> {
    let assignments = service::assignments::get_assignments(&conn, assignee_id).await?;
    Ok(controller::create_success_response_ok(assignments))
}