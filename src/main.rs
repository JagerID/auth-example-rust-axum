use app::app;
use config::{env::load_env, tracing::init_tracing};
use db::Database;
use state::app_state;
use std::sync::Arc;

mod app;
mod config;
mod db;
mod state;
mod swagger;
mod web;

#[tokio::main]
async fn main() {
    init_tracing();

    let env = load_env();

    let postgres = Database::connect_to_postgres(&env.database_url).await;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
        .await
        .unwrap();

    let state = Arc::new(app_state(postgres, env));

    let app = app(state).await;

    axum::serve(listener, app).await.unwrap();
}
