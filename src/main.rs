use app::app;
use config::{env::load_env, tracing::init_tracing};
use db::scylla::Scylla;
use state::app_state;
use std::error::Error;

mod app;
mod config;
mod db;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_tracing();

    let env = load_env();

    let scylla = Scylla::connect(&env.scylla_host, &env.scylla_port).await?;
    scylla.migrate().await?;

    let app = app().await;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
        .await
        .unwrap();

    app_state(scylla, env);

    axum::serve(listener, app).await.unwrap();

    Ok(())
}
