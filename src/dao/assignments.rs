use crate::{models::Assignment, TiraDbConn};
use chrono::Utc;
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

/// DAO function for updating assignments by ticket id.
pub async fn update_assignments_by_ticket_id(
    conn: &TiraDbConn,
    ticket_id: i64,
    assignee_ids: Vec<i64>,
    assigner_id: i64,
) -> QueryResult<usize> {
    use crate::schema::assignments;

    conn.run(move |c| {
        diesel::delete(assignments::table)
            .filter(assignments::ticket_id.eq(ticket_id))
            .execute(c)?;

        let assignments_parameter: Vec<_> = assignee_ids
            .iter()
            .map(|assignee_id| {
                (
                    assignments::ticket_id.eq(ticket_id),
                    assignments::assignee_id.eq(assignee_id),
                    assignments::assigner_id.eq(assigner_id),
                    assignments::assigned.eq(Utc::now().naive_utc()),
                )
            })
            .collect();

        diesel::insert_into(assignments::table)
            .values(&assignments_parameter)
            .execute(c)
    })
    .await
}
