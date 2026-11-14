#![allow(unused_variables, dead_code)]

use sqlx::PgPool;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError {
    #[error("DATABASE_URL is missing")]
    MissingDatabaseUrl,
    #[error("database URL must not be empty")]
    EmptyDatabaseUrl,
    #[error("max connections must be greater than zero")]
    InvalidMaxConnections,
    #[error("DB_MAX_CONNECTIONS is invalid: {0}")]
    InvalidMaxConnectionsText(String),
}

impl DatabaseConfig {
    /// Проверяет непустой URL и max_connections > 0, сохраняя trimmed URL.
    pub fn new(url: String, max_connections: u32) -> Result<Self, ConfigError> {
        todo!("проверить конфигурацию")
    }

    /// Читает DATABASE_URL и DB_MAX_CONNECTIONS (по умолчанию 5).
    pub fn from_env() -> Result<Self, ConfigError> {
        todo!("прочитать конфигурацию окружения без логирования URL")
    }
}

/// Создаёт PostgreSQL pool через PgPoolOptions.
pub async fn connect(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error> {
    todo!("настроить max_connections и подключиться")
}

/// Выполняет SELECT 1::BIGINT.
pub async fn healthcheck(pool: &PgPool) -> Result<i64, sqlx::Error> {
    todo!("выполнить параметризованный по форме, простой health query")
}

/// Применяет миграции из ./migrations.
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    todo!("запустить sqlx::migrate!")
}
