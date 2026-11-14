use std::time::Duration;

use crate::domain::AppError;

#[derive(Clone)]
pub struct AppConfig {
    pub bind_address: String,
    pub database_url: String,
    pub database_max_connections: u32,
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
    pub s3_endpoint: String,
    pub s3_region: String,
    pub s3_bucket: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
    pub s3_force_path_style: bool,
    pub notification_url: String,
    pub notification_api_key: String,
    pub notification_timeout: Duration,
    pub notification_max_retries: u32,
    pub max_attachment_bytes: usize,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AppConfig")
            .field("bind_address", &self.bind_address)
            .field("database_url", &"[REDACTED]")
            .field("s3_endpoint", &self.s3_endpoint)
            .field("s3_bucket", &self.s3_bucket)
            .field("s3_access_key_id", &"[REDACTED]")
            .field("s3_secret_access_key", &"[REDACTED]")
            .field("jwt_secret", &"[REDACTED]")
            .field("notification_api_key", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        todo!("прочитать обязательные переменные и значения по умолчанию из .env.example")
    }

    pub fn validate(&self) -> Result<(), AppError> {
        todo!("URL/credentials/TTL/limits и запрет пустых значений")
    }
}
