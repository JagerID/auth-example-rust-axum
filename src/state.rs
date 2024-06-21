use crate::{config::env::Env, db::postgres::PostgresPool};

#[derive(Debug)]
pub struct AppState {
    pub db: PostgresPool,
    pub env: Env,
}

pub fn app_state(db: PostgresPool, env: Env) -> AppState {
    AppState { db, env }
}
