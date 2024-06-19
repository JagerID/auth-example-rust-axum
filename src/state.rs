use crate::{config::env::Env, db::scylla::Scylla};

pub struct AppState {
    pub db: Scylla,
    pub env: Env,
}

pub fn app_state(db: Scylla, env: Env) -> AppState {
    AppState { db, env }
}
