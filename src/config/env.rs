use serde::Deserialize;
use tracing::error;

fn default_port() -> u16 {
    8000
}

#[derive(Debug, Deserialize, Clone)]
pub struct Env {
    #[serde(default = "default_port")]
    pub port: u16,

    pub database_url: String,

    pub jwt_secret: String,
    pub jwt_token_exp: i64,
    pub jwt_refresh_exp: i64,
}

pub fn load_env() -> Env {
    dotenv::dotenv().ok();

    match envy::from_env::<Env>() {
        Ok(env) => env,
        Err(error) => {
            error!("{:#?}", error);
            std::process::exit(1);
        }
    }
}
