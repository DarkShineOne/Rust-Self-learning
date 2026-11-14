#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CurrentUser {
    pub id: u64,
    pub role: Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackStatus {
    New,
    InReview,
    Resolved,
    Rejected,
}

impl FeedbackStatus {
    pub fn can_transition_to(self, next: Self) -> bool {
        todo!("same + New/InReview workflow из предыдущих уроков")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Feedback {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
    pub status: FeedbackStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AuthzError {
    #[error("feedback not found")]
    NotFound,
    #[error("forbidden")]
    Forbidden,
    #[error("invalid status transition")]
    InvalidTransition,
}

pub fn require_user(actor: &CurrentUser) -> Result<(), AuthzError> {
    todo!("только Role::User")
}

pub fn require_moderator(actor: &CurrentUser) -> Result<(), AuthzError> {
    todo!("Moderator или Admin")
}

/// User видит только своё (чужое → NotFound), модераторы — всё.
pub fn authorize_view(actor: &CurrentUser, feedback: &Feedback) -> Result<(), AuthzError> {
    todo!("проверка владельца без раскрытия чужого ресурса")
}

/// Public update доступен только User-владельцу и только в статусе New.
pub fn authorize_user_update(actor: &CurrentUser, feedback: &Feedback) -> Result<(), AuthzError> {
    todo!("role + ownership + status")
}

/// Разрешает модераторский переход только staff и по workflow.
pub fn authorize_moderation(
    actor: &CurrentUser,
    from: FeedbackStatus,
    to: FeedbackStatus,
) -> Result<(), AuthzError> {
    todo!("require_moderator + can_transition_to")
}

#[derive(Clone)]
pub struct AppState {
    pub feedback: Arc<Feedback>,
}

impl AppState {
    pub fn new(feedback: Feedback) -> Self {
        todo!("обернуть учебную запись в Arc")
    }
}

#[derive(Debug)]
pub struct ApiError(pub AuthzError);

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self.0 {
            AuthzError::NotFound => StatusCode::NOT_FOUND,
            AuthzError::Forbidden => StatusCode::FORBIDDEN,
            AuthzError::InvalidTransition => StatusCode::CONFLICT,
        };
        (
            status,
            Json(serde_json::json!({ "error": self.0.to_string() })),
        )
            .into_response()
    }
}

/// Public route: только User, затем ownership-scoped read.
pub async fn public_feedback(
    State(state): State<AppState>,
    Extension(actor): Extension<CurrentUser>,
    Path(id): Path<u64>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("require_user + id + authorize_view")
}

/// Private route: Moderator/Admin могут читать любую запись.
pub async fn private_feedback(
    State(state): State<AppState>,
    Extension(actor): Extension<CurrentUser>,
    Path(id): Path<u64>,
) -> Result<Json<Feedback>, ApiError> {
    todo!("require_moderator + id + authorize_view")
}

pub fn public_app(state: AppState) -> Router {
    todo!("GET /feedback/{{id}} для пользователей")
}

pub fn private_app(state: AppState) -> Router {
    todo!("GET /feedback/{{id}} для модераторов")
}
