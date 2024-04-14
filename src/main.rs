use crate::controller::authentication;
use crate::service::emails::handle_emails;
use crate::service::emails::Email;
use axum::middleware;
use axum::routing::delete;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;
use axum::Router;
use clap::Parser;
use dotenv::dotenv;
use log::info;
use std::sync::mpsc;
use tokio::net::TcpListener;
use tokio::task;
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
mod controller;
mod dao;
mod models;
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

    // Import environment variables from .env file
    dotenv().ok();

    let args = Args::parse();

    let (email_tx, email_rx) = mpsc::sync_channel(512);
    // Listen for emails on the email queue
    task::spawn(async move {
        handle_emails(email_rx);
    });

    let state = TiraState {
        email_tx: email_tx,
        pool: PgPoolOptions::new().connect(&args.database_url).await?,
    };

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let no_auth_routes = Router::new()
        .route("/login", post(controller::sessions::login_endpoint))
        .layer(cors.clone())
        .with_state(state.clone());

    let auth_routes = Router::new()
        .route(
            "/assignments",
            get(controller::assignments::get_assignments_endpoint),
        )
        .route(
            "/categories",
            delete(controller::categories::archive_category_by_id_endpoint),
        )
        .route(
            "/categories",
            post(controller::categories::create_category_endpoint),
        )
        .route(
            "/categories",
            get(controller::categories::get_categories_endpoint),
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
            post(controller::images::upload_image_endpoint),
        )
        .route(
            "/images/:file_name",
            get(controller::images::retrieve_image_endpoint),
        )
        .route("/logout", post(controller::sessions::logout_endpoint))
        .route(
            "/tickets/:ticket_id/assignments",
            post(controller::tickets::create_assignment_by_ticket_id_endpoint),
        )
        .route(
            "/tickets/:ticket_id/comments",
            post(controller::tickets::create_comment_by_ticket_id_endpoint),
        )
        .route(
            "/tickets",
            post(controller::tickets::create_ticket_endpoint),
        )
        .route(
            "/tickets/:ticket_id/assignments",
            get(controller::tickets::get_assignments_by_ticket_id_endpoint),
        )
        .route(
            "/tickets/:ticket_id/comments",
            get(controller::tickets::get_comments_by_ticket_id_endpoint),
        )
        .route(
            "/tickets/:ticket_id",
            get(controller::tickets::get_ticket_by_id_endpoint),
        )
        .route("/tickets", get(controller::tickets::get_tickets_endpoint))
        .route(
            "/tickets/:ticket_id",
            patch(controller::tickets::patch_ticket_by_id_endpoint),
        )
        .route(
            "/users/:user_id",
            delete(controller::users::archive_user_by_id_endpoint),
        )
        .route("/users", post(controller::users::create_user_endpoint))
        .route(
            "/users/:user_id/assignments",
            get(controller::users::get_assignments_by_user_id_endpoint),
        )
        .route(
            "/users/current",
            get(controller::users::get_current_user_endpoint),
        )
        .route(
            "/users/:user_id",
            get(controller::users::get_user_by_id_endpoint),
        )
        .route("/users", get(controller::users::get_users_endpoint))
        .route(
            "/users/:user_id",
            patch(controller::users::patch_user_by_id_endpoint),
        )
        .layer(cors)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        ))
        .with_state(state);

    let app = no_auth_routes.merge(auth_routes);

    let bind = format!("0.0.0.0:{}", args.port);
    let listener = TcpListener::bind(&bind).await?;

    info!("tira-backend is listening on {}", &bind);
    axum::serve(listener, app).await?;

    Ok(())
}
