use crate::models::patch::UpdateUser;
use crate::models::success::{AlteredResourceResponse, AssignmentResponse};
use crate::models::User;
use crate::models::{Session, TicketWithReporterAsUser};
use crate::service;
use crate::TiraState;
use anyhow::Result;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use serde::{Deserialize, Serialize};

use super::TiraError;

/// Endpoint for archiving a specific user.
//
/// Requires authentication.
///
/// **DELETE /users/<user_id>**

pub async fn archive_user_by_id_endpoint(
    State(state): State<TiraState>,
    Path(user_id): Path<i64>,
) -> Result<Response, TiraError> {
    service::users::archive_user_by_id(&state, user_id).await?;

    let message = "Successfully archived user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: user_id,
    };
    Ok(Json(response).into_response())
}

/// Endpoint for creating a user.
///
/// **POST /users**
///
/// Example JSON Body:
///
/// {
///     "username": "testusername",
///     "password": "testsha256password",
///     "email_address": "testemailaddress",
///     "first_name": "testfirstname",
///     "last_name": "testtestname",
/// }

pub async fn create_user_endpoint(
    State(state): State<TiraState>,
    Json(mut user): Json<User>,
) -> Result<Response, TiraError> {
    user.password = service::security::sha256(&user.password);
    let created_user_id = service::users::create_user(&state, user).await?;
    let message = "Successfully created user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: created_user_id,
    };
    Ok(Json(response).into_response())
}

/// Endpoint for retrieving all assignments for a user.
///
/// **GET /users/<user_id>/assignments**

pub async fn get_assignments_by_user_id_endpoint(
    State(state): State<TiraState>,
    Path(user_id): Path<i64>,
) -> Result<Response, TiraError> {
    let assignments = service::users::get_assignments_by_user_id(&state, user_id).await?;

    let mut ticket_ids = Vec::new();
    let mut assigner_ids = Vec::new();

    for assignment in &assignments {
        ticket_ids.push(assignment.ticket_id);
        assigner_ids.push(assignment.assigner_id);
    }

    let tickets = service::tickets::get_tickets_by_ids(&state, ticket_ids).await?;
    let assigners = service::users::get_users_by_ids(&state, assigner_ids).await?;

    let mut ticket_reporter_ids = Vec::new();

    for ticket in &tickets {
        ticket_reporter_ids.push(ticket.reporter_id);
    }

    let ticket_reporters = service::users::get_users_by_ids(&state, ticket_reporter_ids).await?;

    let mut assignment_responses = Vec::new();
    for (index, assignment) in assignments.iter().enumerate() {
        let ticket = &tickets[index];
        let ticket_with_reporter_as_user = TicketWithReporterAsUser {
            id: ticket.id,
            subject: ticket.subject.clone(),
            description: ticket.description.clone(),
            category_id: ticket.category_id,
            priority: ticket.priority.clone(),
            status: ticket.status.clone(),
            created: ticket.created,
            reporter: ticket_reporters[index].clone(),
        };

        assignment_responses.push(AssignmentResponse {
            id: assignment.id,
            ticket: ticket_with_reporter_as_user,
            assigner: assigners[index].clone(),
            assigned: assignment.assigned,
        });
    }

    Ok(Json(assignment_responses).into_response())
}

/// Endpoint for retrieving the current user.
///
/// Requires authentication.
///
/// **GET /users/current**

pub async fn get_current_user_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
) -> Result<Response, TiraError> {
    let user = service::users::get_user_by_id(&state, session.user_id).await?;
    Ok(Json(user).into_response())
}

/// Endpoint for retrieving a user.
///
/// **GET /users/<user_id>**

pub async fn get_user_by_id_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
) -> Result<Response, TiraError> {
    let user = service::users::get_user_by_id(&state, session.user_id).await?;
    Ok(Json(user).into_response())
}

#[derive(Serialize, Deserialize)]
pub struct GetUsersQueryParameters {
    archived: Option<bool>,
}

/// Endpoint for retrieving every user.
///
/// **GET /users**
///
/// Query Parameters:
///
/// archived: Used to filter users that are archived or not. Takes a boolean value. (optional)
pub async fn get_users_endpoint(
    State(state): State<TiraState>,
    Query(query): Query<GetUsersQueryParameters>,
) -> Result<Response, TiraError> {
    let users = service::users::get_users(&state, query.archived).await?;
    Ok(Json(users).into_response())
}

/// Endpoint for updating a user.
///
/// **PATCH /users/<user_id>**
pub async fn patch_user_by_id_endpoint(
    State(state): State<TiraState>,
    Extension(session): Extension<Session>,
    Path(user_id): Path<i64>,
    Json(user): Json<UpdateUser>,
) -> Result<Response, TiraError> {
    if user_id != session.user_id {
        return Ok((
            StatusCode::BAD_REQUEST,
            "Cannot edit another person's user!",
        )
            .into_response());
    }

    service::users::update_user_by_id(&state, user, session.user_id).await?;

    let message = "Successfully edited user!".to_string();
    let response = AlteredResourceResponse {
        message,
        id: session.user_id,
    };
    Ok(Json(response).into_response())
}
