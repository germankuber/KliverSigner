use std::{env, fmt};
use starknet_types_core::felt::Felt;

#[derive(Clone)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub api_key: String,
    pub starknet_private_key: Felt,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        // dotenv loaded in main
        let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(3000);

        let api_key = env::var("API_KEY").map_err(|_| ConfigError::Missing("API_KEY"))?;
        let starknet_private_key = parse_felt_env("STARKNET_PRIVATE_KEY")?;

        Ok(Self { host, port, api_key, starknet_private_key })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("missing required env var: {0}")]
    Missing(&'static str),
    #[error("invalid value for env var: {0}")]
    Invalid(&'static str),
}

fn parse_felt_env(key: &'static str) -> Result<Felt, ConfigError> {
    let raw = env::var(key).map_err(|_| ConfigError::Missing(key))?;
    let s = raw.strip_prefix("0x").unwrap_or(&raw);
    Felt::from_hex(s).map_err(|_| ConfigError::Invalid(key))
}

impl fmt::Debug for AppConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AppConfig")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("api_key", &"<redacted>")
            .field("starknet_private_key", &"<redacted>")
            .finish()
    }
}
