mod db;
mod handlers;
mod models;

use axum::{routing::{delete, get, post, put}, Router};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt::init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    tracing::info!("Connected to PostgreSQL");

    let tables_exist = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (SELECT FROM information_schema.tables WHERE table_name = 'periodos')"
    )
        .fetch_one(&pool)
        .await
        .unwrap_or(false);

    if !tables_exist {
        tracing::error!("Tables not found in database. Please run the migrations first:");
        tracing::error!("  1. Open Supabase SQL Editor");
        tracing::error!("  2. Run migrations/001_init.sql");
        tracing::error!("  3. Run migrations/002_seed.sql");
        std::process::exit(1);
    }

    tracing::info!("Database tables verified");

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

    let frontend_dir = std::env::var("FRONTEND_DIR").unwrap_or_else(|_| "../frontend".into());
    tracing::info!("Serving frontend from: {frontend_dir}");

    let app = Router::new()
        .nest("/api", api)
        .fallback_service(ServeDir::new(&frontend_dir))
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