use crate::error::AppError;
use std::env;

#[derive(Clone, Debug)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| AppError::InternalServerError("DATABASE_URL must be set".into()))?;

        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| AppError::InternalServerError("JWT_SECRET must be set".into()))?;

        let port: u16 = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            .map_err(|_| AppError::InternalServerError("PORT must be a valid u16".into()))?;

        Ok(Self {
            database_url,
            jwt_secret,
            port,
        })
    }
}
