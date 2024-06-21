use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{error, info};

use super::Database;

pub type PostgresPool = Pool<Postgres>;

impl Database {
    pub async fn connect_to_postgres(database_url: &str) -> PostgresPool {
        match PgPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
        {
            Ok(pool) => {
                info!("✅ Connected to database (Postgres)");
                pool
            }
            Err(_) => {
                error!("✅ Failed connect  database");
                std::process::exit(1);
            }
        }
    }
}
