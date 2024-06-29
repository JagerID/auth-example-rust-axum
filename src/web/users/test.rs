#[tokio::test]
async fn get_users_test() {
    let env = crate::config::env::load_env();

    let postgres = crate::db::Database::connect_to_postgres(&env.database_url).await;

    let state = std::sync::Arc::new(crate::state::app_state(postgres, env));

    let app = crate::app::app(state).await;

    let server = axum_test::TestServer::new(app).unwrap();

    let response = server.get("/api/users").await;

    assert_eq!(response.text(), "[]");
}
