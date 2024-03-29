use crate::controller::{self, TiraMessage, TiraResponse};
use crate::models::patch::UpdateTicket;
use crate::models::success::{
    AlteredResourceResponse, CommentResponse, CountResponse, TicketResponse,
    TicketWithoutDescriptionResponse,
};
use crate::models::{
    create::{CreateAssignmentWithUserId, CreateComment, CreateTicket},
    Assignment,
};
use crate::service::{self, tickets};
use crate::TiraDbConn;
use crate::TiraState;
use rocket::http::{CookieJar, Status};
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::State;

/// Endpoint for creating an assignment for a ticket.
///
/// Requires authentication.
///
/// **POST /tickets/<ticket_id>/assignments**
///
/// Example JSON Body:
///
/// {
///     "user_id": "123"
/// }
#[post("/tickets/<ticket_id>/assignments", data = "<create_assignment_json>")]
pub async fn create_assignment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_assignment_json: Json<CreateAssignmentWithUserId>,
    ticket_id: i64,
    state: &State<TiraState>,
) -> TiraResponse<AlteredResourceResponse> {
    let (user_id, _session_uuid) = controller::authentication(&conn, cookies).await?;
    let assignee_id = create_assignment_json.assignee_id;
    let created_assignment_id = tickets::create_assignment_by_ticket_id_and_assigner_id(
        &conn,
        create_assignment_json.0,
        ticket_id,
        user_id,
    )
    .await?;

    let assignee = service::users::get_user_by_id(&conn, assignee_id).await?;

    if let Some(email_address) = assignee.email_address {
        let assigner = service::users::get_user_by_id(&conn, user_id).await?;
        let ticket = service::tickets::get_ticket_by_id(&conn, ticket_id).await?;

        let body =
            service::emails::create_assignment_email_body(&assigner, &ticket.subject, ticket.id);

        state
            .email_tx
            .send(service::emails::Email {
                to: email_address,
                subject: ticket.subject,
                body,
            })
            .unwrap();
    }

    let message = "Successfully created assignment!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: created_assignment_id,
    };
    Ok(controller::create_success_response(
        Status::Created,
        response,
    ))
}

/// Endpoint for creating a comment for a ticket.
///
/// Requires authentication.
///
/// **POST /tickets/<ticket_id>/comments**
///
/// Example JSON Body:
///
/// {
///     "content": "This is a comment"
/// }
#[post("/tickets/<ticket_id>/comments", data = "<create_comment_json>")]
pub async fn create_comment_by_ticket_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_comment_json: Json<CreateComment>,
    ticket_id: i64,
    state: &State<TiraState>,
) -> TiraResponse<AlteredResourceResponse> {
    let (commenter_id, _session_uuid) = controller::authentication(&conn, cookies).await?;
    let commenter = service::users::get_user_by_id(&conn, commenter_id).await?;
    let content = create_comment_json.0.content.clone();

    let created_comment_id = tickets::create_comment_by_ticket_id_and_commenter_id(
        &conn,
        create_comment_json.0,
        ticket_id,
        commenter_id,
    )
    .await?;

    // Email all users about new comment (except for commenter)
    let users = service::users::get_users(&conn, None).await?;
    let ticket = service::tickets::get_ticket_by_id(&conn, ticket_id).await?;
    for user in users {
        if user.id != commenter_id {
            if let Some(email_address) = user.email_address {
                let body = service::emails::create_comment_email_body(
                    &commenter,
                    &content,
                    &ticket.subject,
                    ticket_id,
                );

                state
                    .email_tx
                    .send(service::emails::Email {
                        to: email_address,
                        subject: ticket.subject.clone(),
                        body,
                    })
                    .unwrap();
            }
        }
    }

    let message = "Successfully created comment!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: created_comment_id,
    };
    Ok(controller::create_success_response(
        Status::Created,
        response,
    ))
}

/// Endpoint for creating a ticket.
///
/// Requires authentication.
///
/// **POST /tickets**
///
/// Example JSON Body:
///
/// {
///     "category_id": "123",
///     "subject": "Finish Tira",
///     "description": "Finish working on the code for Tira",
///     "status": "IN PROGRESS",
///     "priority": "3"
/// }
#[post("/tickets", data = "<create_ticket_json>")]
pub async fn create_ticket_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    create_ticket_json: Json<CreateTicket>,
    state: &State<TiraState>,
) -> Result<Custom<Json<AlteredResourceResponse>>, Custom<Json<TiraMessage>>> {
    let create_ticket = create_ticket_json.0;

    let (reporter_id, _session_uuid) = controller::authentication(&conn, cookies).await?;
    let created_ticket_id =
        service::tickets::create_ticket_by_reporter_id(&conn, create_ticket.clone(), reporter_id)
            .await?;

    let reporter = service::users::get_user_by_id(&conn, reporter_id).await?;

    // Email users about new ticket (except for reporter)
    let users = service::users::get_users(&conn, Some(false)).await?;
    for user in users {
        if user.id != reporter_id {
            if let Some(email_address) = user.email_address {
                let body = service::emails::create_ticket_creation_email_body(
                    &reporter,
                    &create_ticket.subject,
                    &create_ticket
                        .description
                        .clone()
                        .map_or(String::new(), |description| {
                            format!("<p>{}</p>", description)
                        }),
                    created_ticket_id,
                );

                state
                    .email_tx
                    .send(service::emails::Email {
                        to: email_address,
                        subject: create_ticket.subject.clone(),
                        body,
                    })
                    .unwrap();
            }
        }
    }

    // Email assignees about new ticket (except for reporter if self assigned)
    let assignee_ids: Vec<_> = create_ticket.assignee_ids;
    if !assignee_ids.is_empty() {
        let assignees = service::users::get_users_by_ids(&conn, assignee_ids).await?;

        for assignee in assignees {
            if assignee.id != reporter_id {
                if let Some(email_address) = assignee.email_address {
                    let body = service::emails::create_assignment_email_body(
                        &reporter,
                        &create_ticket.subject,
                        created_ticket_id,
                    );
                    state
                        .email_tx
                        .send(service::emails::Email {
                            to: email_address,
                            subject: create_ticket.subject.clone(),
                            body,
                        })
                        .unwrap();
                }
            }
        }
    }

    let message = "Successfully created ticket!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: created_ticket_id,
    };
    Ok(controller::create_success_response(
        Status::Created,
        response,
    ))
}

/// Endpoint for retrieving all assignments for a ticket.
///
/// **GET /tickets/<ticket_id>/assignments**
#[get("/tickets/<ticket_id>/assignments")]
pub async fn get_assignments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> TiraResponse<Vec<Assignment>> {
    let assignments = service::tickets::get_assignments_by_ticket_id(&conn, ticket_id).await?;
    Ok(controller::create_success_response_ok(assignments))
}

/// Endpoint for retrieving all comments for a ticket.
///
/// **GET /tickets/<ticket_id>/comments**
#[get("/tickets/<ticket_id>/comments")]
pub async fn get_comments_by_ticket_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> TiraResponse<Vec<CommentResponse>> {
    let comments = service::tickets::get_comments_by_ticket_id(&conn, ticket_id).await?;
    let mut comments_response = Vec::new();

    for comment in comments {
        let commenter = service::users::get_user_by_id(&conn, comment.commenter_id).await?;

        let comment_response = CommentResponse {
            id: comment.id,
            commenter,
            content: comment.content,
            commented: comment.commented,
        };

        comments_response.push(comment_response);
    }

    Ok(controller::create_success_response_ok(comments_response))
}

/// Endpoint for retrieving a ticket.
///
/// **GET /tickets/<ticket_id>**
#[get("/tickets/<ticket_id>")]
pub async fn get_ticket_by_id_endpoint(
    conn: TiraDbConn,
    ticket_id: i64,
) -> TiraResponse<TicketResponse> {
    let ticket = service::tickets::get_ticket_by_id(&conn, ticket_id).await?;

    let category = if let Some(category_id) = ticket.category_id {
        Some(service::categories::get_category_by_id(&conn, category_id).await?)
    } else {
        None
    };
    let reporter = service::users::get_user_by_id(&conn, ticket.reporter_id).await?;

    let assignments = service::assignments::get_assignments(&conn, None, Some(ticket.id)).await?;

    let assignee_ids: Vec<_> = assignments
        .iter()
        .map(|assignment| assignment.assignee_id)
        .collect();

    let assignees = service::users::get_users_by_ids(&conn, assignee_ids).await?;

    let ticket_response = TicketResponse {
        id: ticket.id,
        subject: ticket.subject,
        description: ticket.description,
        category,
        priority: ticket.priority,
        status: ticket.status,
        created: ticket.created,
        reporter,
        assignees,
    };

    Ok(controller::create_success_response_ok(ticket_response))
}

/// Endpoint for retrieving every ticket.
///
/// **GET /tickets**
///
/// Query Parameters:
///
/// limit: How many tickets should be retrieved (optional, default is 10)
/// offset: The offset for the list of tickets (optional, default is 0)
/// reporter: Used to filter tickets that were reported by a certain user. Takes a number value. (optional)
/// open: Used to filter tickets that are open or not. Takes a boolean. (optional)
#[get("/tickets?<limit>&<offset>&<reporter>&<open>&<sort_by>&<order_by>")]
pub async fn get_tickets_endpoint(
    conn: TiraDbConn,
    limit: Option<i64>,
    offset: Option<i64>,
    reporter: Option<i64>,
    open: Option<bool>,
    sort_by: Option<String>,
    order_by: Option<String>,
) -> TiraResponse<CountResponse<TicketWithoutDescriptionResponse>> {
    let (tickets, total_count) =
        tickets::get_tickets(&conn, limit, offset, reporter, open, sort_by, order_by).await?;
    let mut tickets_response = Vec::new();

    for ticket in tickets {
        let reporter = service::users::get_user_by_id(&conn, ticket.reporter_id).await?;

        let category = match ticket.category_id {
            Some(category_id) => {
                Some(service::categories::get_category_by_id(&conn, category_id).await?)
            }
            None => None,
        };

        let assignments =
            service::assignments::get_assignments(&conn, None, Some(ticket.id)).await?;

        let assignee_ids: Vec<_> = assignments
            .iter()
            .map(|assignment| assignment.assignee_id)
            .collect();

        let assignees = service::users::get_users_by_ids(&conn, assignee_ids).await?;

        let ticket_response = TicketWithoutDescriptionResponse {
            id: ticket.id,
            subject: ticket.subject.clone(),
            category,
            priority: ticket.priority.clone(),
            status: ticket.status.clone(),
            created: ticket.created,
            reporter,
            assignees,
        };

        tickets_response.push(ticket_response);
    }

    let response = CountResponse {
        data: tickets_response,
        total_count,
    };

    Ok(controller::create_success_response_ok(response))
}

/// Endpoint for updating a ticket.
///
/// **PATCH /tickets/<ticket_id>**
#[patch("/tickets/<ticket_id>", data = "<update_ticket_json>")]
pub async fn patch_ticket_by_id_endpoint(
    conn: TiraDbConn,
    cookies: &CookieJar<'_>,
    update_ticket_json: Json<UpdateTicket>,
    ticket_id: i64,
) -> TiraResponse<AlteredResourceResponse> {
    let (user_id, _session_uuid) = controller::authentication(&conn, cookies).await?;

    service::tickets::update_ticket_by_id(&conn, update_ticket_json.0.clone(), ticket_id).await?;
    if let Some(assignee_ids) = update_ticket_json.0.assignee_ids {
        service::tickets::update_assignments_by_ticket_id(&conn, ticket_id, assignee_ids, user_id)
            .await?;
    }

    let message = "Successfully edited ticket!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: ticket_id,
    };
    Ok(controller::create_success_response_ok(response))
}
