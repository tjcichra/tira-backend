use crate::{models::Assignment, TiraDbConn};
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};

/// DAO function for retrieving all assignments.
pub async fn get_assignments(
    conn: &TiraDbConn,
    assignee_id: Option<i64>,
    ticket_id: Option<i64>,
) -> QueryResult<Vec<Assignment>> {
    use crate::schema::assignments;

    conn.run(move |c| {
        let mut query = assignments::table.into_boxed();

        if let Some(assignee_id) = assignee_id {
            query = query.filter(assignments::assignee_id.eq(assignee_id));
        }

        if let Some(ticket_id) = ticket_id {
            query = query.filter(assignments::ticket_id.eq(ticket_id));
        }

        query.load(c)
    })
    .await
}
