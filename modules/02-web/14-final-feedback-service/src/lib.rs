#![allow(unused_variables, dead_code, unused_imports)]

pub mod api;
pub mod auth;
pub mod config;
pub mod domain;
pub mod external;
pub mod repository;
pub mod service;
pub mod storage;

pub use api::{AppState, app, private_router, public_router};
pub use domain::*;

/// Собирает зависимости, применяет миграции и запускает HTTP server.
pub async fn run() -> Result<(), AppError> {
    todo!("config → PostgreSQL/S3/HTTP clients → migrations → app → graceful shutdown")
}
