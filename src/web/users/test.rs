use std::sync::Arc;

use axum_test::TestServer;

use super::dto::FilteredUser;
use crate::{app::app, config::env::load_env, db::Database, state::app_state};

#[tokio::test]
async fn get_users_test() {
    let env = load_env();

    let postgres = Database::connect_to_postgres(&env.database_url).await;

    let state = Arc::new(app_state(postgres, env));

    let app = app(state).await;

    let server = TestServer::new(app).unwrap();

    let response = server.get("/api/v1/users").await;

    assert_eq!(response.text(), "[]");
}
