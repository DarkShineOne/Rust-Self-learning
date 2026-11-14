#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use async_trait::async_trait;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub rating: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NewFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateFeedback {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ServiceError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("body must not be empty")]
    EmptyBody,
    #[error("rating must be between 1 and 5")]
    InvalidRating,
    #[error("feedback not found")]
    NotFound,
}

#[async_trait]
pub trait FeedbackRepository: Send + Sync {
    async fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError>;
    async fn list(&self) -> Result<Vec<Feedback>, ServiceError>;
    async fn get(&self, id: u64) -> Result<Feedback, ServiceError>;
    async fn replace(&self, feedback: Feedback) -> Result<Feedback, ServiceError>;
    async fn delete(&self, id: u64) -> Result<Feedback, ServiceError>;
}

#[derive(Debug)]
pub struct MemoryRepository {
    records: RwLock<HashMap<u64, Feedback>>,
    next_id: AtomicU64,
}

impl MemoryRepository {
    /// Создаёт пустой репозиторий, следующий id равен 1.
    pub fn new() -> Self {
        todo!("инициализировать состояние")
    }
}

impl Default for MemoryRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl FeedbackRepository for MemoryRepository {
    async fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError> {
        todo!("атомарно выделить id и добавить запись")
    }

    async fn list(&self) -> Result<Vec<Feedback>, ServiceError> {
        todo!("вернуть клоны записей по возрастанию id")
    }

    async fn get(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("вернуть клон записи или NotFound")
    }

    async fn replace(&self, feedback: Feedback) -> Result<Feedback, ServiceError> {
        todo!("заменить только существующую запись")
    }

    async fn delete(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("удалить и вернуть запись")
    }
}

pub struct FeedbackService<R> {
    repository: Arc<R>,
}

impl<R: FeedbackRepository> FeedbackService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        todo!("сохранить Arc repository")
    }

    pub async fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError> {
        todo!("trim + валидация, затем repository.create")
    }

    pub async fn list(&self) -> Result<Vec<Feedback>, ServiceError> {
        todo!("делегировать repository.list")
    }

    pub async fn get(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("делегировать repository.get")
    }

    pub async fn update(&self, id: u64, update: UpdateFeedback) -> Result<Feedback, ServiceError> {
        todo!("получить запись, применить Some-поля, провалидировать и replace")
    }

    pub async fn delete(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("делегировать repository.delete")
    }
}

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<FeedbackService<MemoryRepository>>,
}

impl AppState {
    pub fn new() -> Self {
        todo!("собрать MemoryRepository, FeedbackService и AppState")
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn status_code(error: &ServiceError) -> StatusCode {
    todo!("NotFound → 404, ошибки валидации → 422")
}

#[derive(Debug)]
pub struct ApiError {
    pub status: StatusCode,
    pub message: String,
}

impl From<ServiceError> for ApiError {
    fn from(error: ServiceError) -> Self {
        Self {
            status: status_code(&error),
            message: error.to_string(),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status,
            Json(serde_json::json!({ "error": self.message })),
        )
            .into_response()
    }
}

pub async fn create_handler(
    State(state): State<AppState>,
    Json(draft): Json<NewFeedback>,
) -> Result<(StatusCode, Json<Feedback>), ApiError> {
    todo!("201 + созданная запись")
}

pub async fn list_handler(State(state): State<AppState>) -> Result<Json<Vec<Feedback>>, ApiError> {
    todo!("вызвать service.list")
}

pub async fn get_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("вызвать service.get")
}

pub async fn update_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(update): Json<UpdateFeedback>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("вызвать service.update")
}

pub async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("вызвать service.delete")
}

/// Регистрирует CRUD под /api/public/v1/feedback с заданным state.
pub fn app(state: AppState) -> Router {
    todo!("собрать routes и передать state")
}
