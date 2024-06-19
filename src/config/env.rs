use serde::Deserialize;
use tracing::error;

fn default_scylla_port() -> u16 {
    9042
}

fn default_port() -> u16 {
    8000
}

fn scylla_nodes() -> Vec<String> {
    vec![
        "172.21.0.2".to_string(),
        "172.21.0.3".to_string(),
        "172.21.0.4".to_string(),
    ]
}

#[derive(Debug, Deserialize, Clone)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default = "scylla_nodes")]
    pub scylla_nodes: Vec<String>,

    #[serde(default = "default_scylla_port")]
    pub scylla_port: u16,
}

pub fn load_env() -> Env {
    dotenv::dotenv().ok();

    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(err) => {
            error!("{:#?}", err);
            std::process::exit(1);
        }
    }
}
