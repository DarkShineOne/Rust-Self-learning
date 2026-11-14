#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use async_trait::async_trait;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewFeedback {
    pub title: String,
    pub rating: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Notification {
    pub feedback_id: u64,
    pub title: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreationOutcome {
    pub feedback: Feedback,
    pub notification_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ServiceError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("rating must be between 1 and 5")]
    InvalidRating,
    #[error("repository error: {0}")]
    Repository(String),
}

#[async_trait]
pub trait FeedbackRepository: Send + Sync {
    async fn insert(&self, draft: NewFeedback) -> Result<Feedback, ServiceError>;
}

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn notify(&self, notification: Notification) -> Result<(), String>;
}

pub struct FeedbackService {
    repository: Arc<dyn FeedbackRepository>,
    notifier: Arc<dyn Notifier>,
}

impl FeedbackService {
    pub fn new(repository: Arc<dyn FeedbackRepository>, notifier: Arc<dyn Notifier>) -> Self {
        todo!("сохранить trait objects")
    }

    /// Сохраняет feedback, затем пытается отправить уведомление.
    /// Ошибка уведомления не откатывает запись, а возвращается в outcome.
    pub async fn create(&self, draft: NewFeedback) -> Result<CreationOutcome, ServiceError> {
        todo!("validate → insert → notify с явным partial outcome")
    }
}

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<FeedbackService>,
}

impl AppState {
    pub fn new(service: Arc<FeedbackService>) -> Self {
        todo!("сохранить service")
    }
}

#[derive(Debug)]
pub struct ApiError(ServiceError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            ServiceError::EmptyTitle | ServiceError::InvalidRating => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            ServiceError::Repository(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (
            status,
            Json(serde_json::json!({ "error": self.0.to_string() })),
        )
            .into_response()
    }
}

pub async fn create_handler(
    State(state): State<AppState>,
    Json(draft): Json<NewFeedback>,
) -> Result<(StatusCode, Json<CreationOutcome>), ApiError> {
    todo!("201 + service.create")
}

pub fn app(state: AppState) -> Router {
    todo!("POST /api/public/v1/feedback")
}
