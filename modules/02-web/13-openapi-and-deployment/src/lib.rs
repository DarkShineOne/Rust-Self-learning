#![allow(unused_variables, dead_code, unused_imports)]

use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use utoipa::{OpenApi, ToSchema};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct CreateFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, ToSchema)]
pub struct ModerateFeedback {
    pub status: String,
}

#[utoipa::path(
    post,
    path = "/api/public/v1/feedback",
    request_body = CreateFeedback,
    responses((status = 201, description = "Feedback created", body = Feedback))
)]
pub async fn create_feedback(Json(input): Json<CreateFeedback>) -> Json<Feedback> {
    todo!("учебный handler для public OpenAPI")
}

#[utoipa::path(
    patch,
    path = "/api/private/v1/feedback/{id}",
    params(("id" = u64, Path, description = "Feedback id")),
    request_body = ModerateFeedback,
    responses((status = 200, description = "Feedback moderated", body = Feedback))
)]
pub async fn moderate_feedback(Json(input): Json<ModerateFeedback>) -> Json<Feedback> {
    todo!("учебный handler для private OpenAPI")
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Feedback Public API", version = "0.1.0"),
    paths(create_feedback),
    components(schemas(CreateFeedback, Feedback))
)]
pub struct PublicApiDoc;

#[derive(OpenApi)]
#[openapi(
    info(title = "Feedback Private API", version = "0.1.0"),
    paths(moderate_feedback),
    components(schemas(ModerateFeedback, Feedback))
)]
pub struct PrivateApiDoc;

pub fn public_openapi() -> utoipa::openapi::OpenApi {
    todo!("PublicApiDoc::openapi()")
}

pub fn private_openapi() -> utoipa::openapi::OpenApi {
    todo!("PrivateApiDoc::openapi()")
}

pub async fn public_docs() -> Json<utoipa::openapi::OpenApi> {
    todo!("вернуть public_openapi")
}

pub async fn private_docs() -> Json<utoipa::openapi::OpenApi> {
    todo!("вернуть private_openapi")
}

/// Регистрирует handlers и раздельные openapi.json endpoints.
pub fn app() -> Router {
    todo!("routes + /api/public/v1/openapi.json + /api/private/v1/openapi.json")
}

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub s3_endpoint: String,
    pub s3_bucket: String,
    pub s3_access_key_id: String,
    pub s3_secret_access_key: String,
    pub jwt_secret: String,
    pub notification_url: String,
}

impl std::fmt::Debug for AppConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AppConfig")
            .field("database_url", &"[REDACTED]")
            .field("s3_endpoint", &self.s3_endpoint)
            .field("s3_bucket", &self.s3_bucket)
            .field("s3_access_key_id", &"[REDACTED]")
            .field("s3_secret_access_key", &"[REDACTED]")
            .field("jwt_secret", &"[REDACTED]")
            .field("notification_url", &self.notification_url)
            .finish()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError {
    #[error("configuration value is empty: {0}")]
    Empty(&'static str),
    #[error("configuration URL is invalid: {0}")]
    InvalidUrl(&'static str),
    #[error("JWT secret must contain at least 32 bytes")]
    WeakJwtSecret,
}

impl AppConfig {
    /// Проверяет наличие секретов и допустимые схемы URL.
    pub fn validate(&self) -> Result<(), ConfigError> {
        todo!("проверить postgres/http(s), bucket/credentials и JWT secret")
    }
}
