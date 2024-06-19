use scylla::{
    transport::errors::{NewSessionError, QueryError},
    Session, SessionBuilder,
};
use tracing::info;

#[derive(Debug)]
pub struct Scylla {
    pub db: Session,
}

impl Scylla {
    pub async fn connect(host: &String, port: &u16) -> Result<Self, NewSessionError> {
        let session = SessionBuilder::new()
            .known_node(format!("{}:{}", host, port))
            .build()
            .await?;

        Ok(Self { db: session })
    }

    pub async fn migrate(&self) -> Result<(), QueryError> {
        self.prepare_keyspace().await?;
        self.create_users_table().await?;

        Ok(())
    }

    async fn prepare_keyspace(&self) -> Result<(), QueryError> {
        self.db
            .query(
                "
                CREATE KEYSPACE IF NOT EXISTS idk
                WITH REPLICATION = { 'class': 'NetworkTopologyStrategy', 'replication_factor': 3 }
            ",
                &[],
            )
            .await?;

        info!("✅️ Keyspace `idk` prepared");

        Ok(())
    }

    async fn create_users_table(&self) -> Result<(), QueryError> {
        let _ = self
            .db
            .query(
                "
                CREATE TABLE IF NOT EXISTS idk.users (
                    id uuid,
                    name varchar,

                    primary key (id)
                )
            ",
                &[],
            )
            .await?;

        info!("✅️ Users table migrated");

        Ok(())
    }
}
