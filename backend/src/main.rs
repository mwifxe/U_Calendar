mod db;
mod handlers;
mod models;

use axum::{routing::{delete, get, post, put}, Router};
use tower_http::cors::{Any, CorsLayer};
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::ConnectOptions;
use std::str::FromStr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let connect_options = PgConnectOptions::from_str(&database_url)
        .expect("Invalid DATABASE_URL")
        .statement_cache_capacity(0);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(connect_options)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api = Router::new()
        .route("/calendario", get(handlers::get_calendario))
        .route("/semanas/{semana_id}/actividades", post(handlers::create_actividad))
        .route("/actividades/{id}", put(handlers::update_actividad))
        .route("/actividades/{id}", delete(handlers::delete_actividad))
        .route("/actividades/{id}/notas", get(handlers::get_nota))
        .route("/actividades/{id}/notas", put(handlers::upsert_nota));

    let app = Router::new()
        .nest("/api", api)
        .layer(cors)
        .with_state(pool);

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "3000".into());
    let addr = format!("{host}:{port}");

    tracing::info!("Server running on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}