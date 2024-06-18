use std::error::Error;

use scylla::{Session, SessionBuilder};
use tracing::{error, info};

use super::DB;

#[derive(Debug)]
pub struct Scylla {
    pub db: Session,
}

impl DB<Scylla> for Scylla {
    async fn connect(port: u16) -> Self {
        let session = SessionBuilder::new()
            .known_node(format!("172.21.0.1:{}", port))
            .build()
            .await
            .unwrap();

        Self { db: session }
    }

    async fn migrate(&self) -> Result<(), Box<dyn Error>> {
        self.db
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
            .await
            .map_err(|e| error!("{}", e));

        let result = self.db.query("SELECT * FROM idk", &[]).await?;

        info!("insert: {:#?}", result);

        Ok(())
    }
}

impl Scylla {
    pub async fn prepare_keyspace(&self) -> Result<(), Box<dyn Error>> {
        self.db
            .query(
                "
                CREATE KEYSPACE IF NOT EXISTS idk
                WITH REPLICATION = { 'class': 'NetworkTopologyStrategy', 'replication_factor': 3 }
            ",
                &[],
            )
            .await?;

        info!("prepared keyspace");

        Ok(())
    }
}
