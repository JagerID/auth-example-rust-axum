use scylla::Session;

use crate::config::env::Env;

#[derive(Debug)]
pub struct AppState {
    pub db: Session,
    pub env: Env,
}

pub fn app_state(db: Session, env: Env) -> AppState {
    AppState { db, env }
}
