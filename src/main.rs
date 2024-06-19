use app::app;
use config::{env::load_env, tracing::init_tracing};
use db::scylla::Scylla;
use state::app_state;
use std::{error::Error, sync::Arc};

mod app;
mod config;
mod db;
mod state;
mod swagger;
mod web;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();

    let env = load_env();

    let scylla = Scylla::connect(&env.scylla_nodes).await?;
    scylla.migrate().await?;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
        .await
        .unwrap();

    let state = Arc::new(app_state(scylla.db, env));

    let app = app(state).await;

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
