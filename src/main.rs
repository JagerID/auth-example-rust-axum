use app::app;
use config::{env::load_env, tracing::init_tracing};
use db::{scylla::Scylla, DB};
use tracing::info;

mod app;
mod config;
mod db;

#[tokio::main]
async fn main() {
    init_tracing();

    let env = load_env();

    let scylla = Scylla::connect(env.scylla_host, env.scylla_port).await;
    info!("SCYLLA: {:#?}", scylla);
    let _ = scylla.prepare_keyspace().await;
    let _ = scylla.migrate().await;

    let res = scylla.db.query("SELECT * FROM idk.users", &[]).await;
    info!("res: {:#?}", res);

    let app = app().await;

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", env.port))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
