use std::env;

use crate::common::error::AppError;

pub enum Config {
    DatabaseUrl,
    JsonWebTokenSecret
}

impl Config {
    pub fn from_env(&self) -> Result<String, Box<dyn std::error::Error>> {
        let key = match self {
            Config::DatabaseUrl => "DATABASE_URL",
            Config::JsonWebTokenSecret => "JWT_SECRET"
        };
        let value = env::var(key).map_err(|e| AppError::Failed(e.to_string()))?;

        Ok(value)
    }
}