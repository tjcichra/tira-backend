use crate::controller::authentication;
use crate::service::emails::handle_emails;
use crate::service::emails::Email;
use axum::body::Body;
use axum::body::Bytes;
use axum::extract::Request;
use axum::http::StatusCode;
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::response::Response;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;
use axum::Router;
use clap::Parser;
use dotenv::dotenv;
use http_body_util::BodyExt;
use log::info;
use std::sync::mpsc;
use std::thread;
use tokio::net::TcpListener;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
mod controller;
mod dao;
mod models;
use std::process;
mod service;
use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};

#[derive(Clone)]
pub struct TiraState {
    pool: PgPool,
    email_tx: mpsc::SyncSender<Email>,
}

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, env)]
    database_url: String,
    #[clap(short, long, env, default_value_t = 8000)]
    port: u16,
}

// The point where the program first starts
#[tokio::main]
async fn main() -> Result<()> {
    simple_logger::init_with_level(log::Level::Info)?;

    ctrlc::set_handler(move || {
        info!("Got signal. Shutting down...");
        process::exit(0);
    })?;

    // Import environment variables from .env file
    info!("reading env");
    dotenv().ok();

    let args = Args::parse();

    info!("setting up email handler");
    let (email_tx, email_rx) = mpsc::sync_channel(512);
    // Listen for emails on the email queue
    thread::spawn(move || {
        handle_emails(email_rx);
    });

    info!("connecting to the database");
    let state = TiraState {
        email_tx,
        pool: PgPoolOptions::new().connect(&args.database_url).await?,
    };
    info!("successfully to the database");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    info!("setting up public routes");
    let no_auth_routes = Router::new()
        .route("/login", post(controller::sessions::login_endpoint))
        .route("/health", get(controller::health))
        .layer(cors.clone())
        .layer(middleware::from_fn(print_request_body))
        .with_state(state.clone());

    info!("setting up private routes");
    let auth_routes = Router::new()
        .route(
            "/assignments",
            get(controller::assignments::get_assignments_endpoint),
        )
        .route(
            "/categories",
            delete(controller::categories::archive_category_by_id_endpoint)
                .post(controller::categories::create_category_endpoint)
                .get(controller::categories::get_categories_endpoint),
        )
        .route(
            "/categories/:category_id",
            get(controller::categories::get_category_by_id_endpoint),
        )
        .route(
            "/comments/:comment_id",
            patch(controller::comments::patch_comment_by_id_endpoint),
        )
        .route(
            "/images/:file_name",
            post(controller::images::upload_image_endpoint)
                .get(controller::images::retrieve_image_endpoint),
        )
        .route("/logout", post(controller::sessions::logout_endpoint))
        .route(
            "/tickets/:ticket_id/assignments",
            post(controller::tickets::create_assignment_by_ticket_id_endpoint),
        )
        .route(
            "/tickets/:ticket_id/comments",
            post(controller::tickets::create_comment_by_ticket_id_endpoint)
                .get(controller::tickets::get_comments_by_ticket_id_endpoint),
        )
        .route(
            "/tickets",
            post(controller::tickets::create_ticket_endpoint)
                .get(controller::tickets::get_tickets_endpoint),
        )
        .route(
            "/tickets/:ticket_id/assignments",
            get(controller::tickets::get_assignments_by_ticket_id_endpoint),
        )
        .route(
            "/tickets/:ticket_id",
            get(controller::tickets::get_ticket_by_id_endpoint)
                .patch(controller::tickets::patch_ticket_by_id_endpoint),
        )
        .route(
            "/users/:user_id",
            delete(controller::users::archive_user_by_id_endpoint)
                .get(controller::users::get_user_by_id_endpoint)
                .patch(controller::users::patch_user_by_id_endpoint),
        )
        .route(
            "/users",
            post(controller::users::create_user_endpoint)
                .get(controller::users::get_users_endpoint),
        )
        .route(
            "/users/:user_id/assignments",
            get(controller::users::get_assignments_by_user_id_endpoint),
        )
        .route(
            "/users/current",
            get(controller::users::get_current_user_endpoint),
        )
        .layer(cors)
        .layer(middleware::from_fn(print_request_body))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        ))
        .with_state(state);

    info!("setting up router");
    let app = no_auth_routes.merge(auth_routes);

    let bind = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&bind).await?;

    info!("tira-backend is listening on {}", &bind);
    axum::serve(listener, app).await?;

    Ok(())
}

// middleware that shows how to consume the request body upfront
async fn print_request_body(request: Request, next: Next) -> Result<impl IntoResponse, Response> {
    info!("Request: {:?}", request);
    let request = buffer_request_body(request).await?;
    Ok(next.run(request).await)
}

// the trick is to take the request apart, buffer the body, do what you need to do, then put
// the request back together
async fn buffer_request_body(request: Request) -> Result<Request, Response> {
    let uri = request.uri().to_string();
    let (parts, body) = request.into_parts();

    // this wont work if the body is an long running stream
    let bytes = body
        .collect()
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response())?
        .to_bytes();

    do_thing_with_request_body(&uri, bytes.clone());

    Ok(Request::from_parts(parts, Body::from(bytes)))
}

fn do_thing_with_request_body(uri: &str, bytes: Bytes) {
    let body = String::from_utf8_lossy(&bytes);
    if !body.is_empty() {
        info!("body for {}: {}", uri, String::from_utf8_lossy(&bytes));
    }
}
