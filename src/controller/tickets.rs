use super::TiraError;
use crate::models::patch::UpdateTicket;
use crate::models::success::{
    AlteredResourceResponse, CommentResponse, CountResponse, TicketResponse,
    TicketWithoutDescriptionResponse,
};
use crate::models::{CreateAssignmentWithUserId, CreateComment, CreateTicket, Session, Ticket};
use crate::service::{self, tickets};
use crate::TiraState;
use anyhow::Result;
use axum::extract::{Path, Query, State};
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::Deserialize;

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
pub async fn create_assignment_by_ticket_id_endpoint(
    State(state): State<TiraState>,
    Path(ticket_id): Path<i64>,
    Extension(session): Extension<Session>,
    Json(assignment): Json<CreateAssignmentWithUserId>,
) -> Result<Response, TiraError> {
    let assignee_id = assignment.assignee_id;
    let created_assignment_id = tickets::create_assignment_by_ticket_id_and_assigner_id(
        &state,
        assignee_id,
        ticket_id,
        session.user_id,
    )
    .await?;

    let assignee = service::users::get_user_by_id(&state, assignee_id).await?;

    if let Some(email_address) = assignee.email_address {
        let assigner = service::users::get_user_by_id(&state, session.user_id).await?;
        let ticket = service::tickets::get_ticket_by_id(&state, ticket_id).await?;

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
    Ok(Json(response).into_response())
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
pub async fn create_comment_by_ticket_id_endpoint(
    State(state): State<TiraState>,
    Path(ticket_id): Path<i64>,
    Extension(session): Extension<Session>,
    Json(comment): Json<CreateComment>,
) -> Result<Response, TiraError> {
    let commenter = service::users::get_user_by_id(&state, session.user_id).await?;

    let created_comment_id = tickets::create_comment_by_ticket_id_and_commenter_id(
        &state,
        &comment.content,
        ticket_id,
        session.user_id,
    )
    .await?;

    // Email all users about new comment (except for commenter)
    let users = service::users::get_users(&state, None).await?;
    let ticket = service::tickets::get_ticket_by_id(&state, ticket_id).await?;
    for user in users {
        if user.id != session.user_id {
            if let Some(email_address) = user.email_address {
                let body = service::emails::create_comment_email_body(
                    &commenter,
                    &comment.content,
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
    Ok(Json(response).into_response())
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

pub async fn create_ticket_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
    Json(ticket): Json<CreateTicket>,
) -> Result<Response, TiraError> {
    let created_ticket_id =
        service::tickets::create_ticket_by_reporter_id(&state, &ticket, session.user_id).await?;

    let reporter = service::users::get_user_by_id(&state, session.user_id).await?;

    // Email users about new ticket (except for reporter)
    let users = service::users::get_users(&state, Some(false)).await?;
    for user in users {
        if user.id != reporter.id {
            if let Some(email_address) = user.email_address {
                let body = service::emails::create_ticket_creation_email_body(
                    &reporter,
                    &ticket.subject,
                    &ticket
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
                        subject: ticket.subject.clone(),
                        body,
                    })
                    .unwrap();
            }
        }
    }

    let message = "Successfully created ticket!".to_string();
    Ok(message.into_response())
}

/// Endpoint for retrieving all assignments for a ticket.
///
/// **GET /tickets/<ticket_id>/assignments**

pub async fn get_assignments_by_ticket_id_endpoint(
    State(state): State<TiraState>,
    Path(ticket_id): Path<i64>,
) -> Result<Response, TiraError> {
    let assignments = service::tickets::get_assignments_by_ticket_id(&state, ticket_id).await?;
    Ok(Json(assignments).into_response())
}

/// Endpoint for retrieving all comments for a ticket.
///
/// **GET /tickets/<ticket_id>/comments**

pub async fn get_comments_by_ticket_id_endpoint(
    State(state): State<TiraState>,
    Path(ticket_id): Path<i64>,
) -> Result<Response, TiraError> {
    let comments = service::tickets::get_comments_by_ticket_id(&state, ticket_id).await?;
    let mut comments_response = Vec::new();

    for comment in comments {
        let commenter = service::users::get_user_by_id(&state, comment.commenter_id).await?;

        let comment_response = CommentResponse {
            id: comment.id,
            commenter,
            content: comment.content,
            commented: comment.commented,
        };

        comments_response.push(comment_response);
    }

    Ok(Json(comments_response).into_response())
}

/// Endpoint for retrieving a ticket.
///
/// **GET /tickets/<ticket_id>**

pub async fn get_ticket_by_id_endpoint(
    State(state): State<TiraState>,
    Path(ticket_id): Path<i64>,
) -> Result<Response, TiraError> {
    let ticket = service::tickets::get_ticket_by_id(&state, ticket_id).await?;

    let category = if let Some(category_id) = ticket.category_id {
        Some(service::categories::get_category_by_id(&state, category_id).await?)
    } else {
        None
    };
    let reporter = service::users::get_user_by_id(&state, ticket.reporter_id).await?;

    let assignments = service::assignments::get_assignments(&state, None, Some(ticket.id)).await?;

    let assignee_ids: Vec<_> = assignments
        .iter()
        .map(|assignment| assignment.assignee_id)
        .collect();

    let assignees = service::users::get_users_by_ids(&state, assignee_ids).await?;

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

    Ok(Json(ticket_response).into_response())
}

#[derive(Deserialize)]
pub struct GetTicketsQueryParams {
    limit: Option<i64>,
    offset: Option<i64>,
    reporter: Option<i64>,
    open: Option<bool>,
    sort_by: Option<String>,
    order_by: Option<String>,
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
pub async fn get_tickets_endpoint(
    State(state): State<TiraState>,
    Query(query): Query<GetTicketsQueryParams>,
) -> Result<Response, TiraError> {
    let (tickets, total_count) = tickets::get_tickets(
        &state,
        query.limit,
        query.offset,
        query.reporter,
        query.open,
        query.sort_by,
        query.order_by,
    )
    .await?;
    let mut tickets_response = Vec::new();

    for ticket in tickets {
        let reporter = service::users::get_user_by_id(&state, ticket.reporter_id).await?;

        let category = match ticket.category_id {
            Some(category_id) => {
                Some(service::categories::get_category_by_id(&state, category_id).await?)
            }
            None => None,
        };

        let assignments =
            service::assignments::get_assignments(&state, None, Some(ticket.id)).await?;

        let assignee_ids: Vec<_> = assignments
            .iter()
            .map(|assignment| assignment.assignee_id)
            .collect();

        let assignees = service::users::get_users_by_ids(&state, assignee_ids).await?;

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

    Ok(Json(response).into_response())
}

/// Endpoint for updating a ticket.
///
/// **PATCH /tickets/<ticket_id>**

pub async fn patch_ticket_by_id_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
    Path(ticket_id): Path<i64>,
    Json(ticket): Json<UpdateTicket>,
) -> Result<Response, TiraError> {
    service::tickets::update_ticket_by_id(&state, &ticket, ticket_id).await?;
    if let Some(assignee_ids) = ticket.assignee_ids {
        service::tickets::update_assignments_by_ticket_id(
            &state,
            ticket_id,
            assignee_ids,
            session.user_id,
        )
        .await?;
    }

    let message = "Successfully edited ticket!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: ticket_id,
    };
    Ok(Json(response).into_response())
}
