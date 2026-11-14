#![allow(unused_variables, dead_code)]

use axum::{Json, Router, http::StatusCode};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Category {
    Bug,
    Idea,
    Question,
    Praise,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub category: Category,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValidatedFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub category: Category,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateFeedback {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ValidationError {
    #[error("title must not be empty")]
    EmptyTitle,
    #[error("title must contain at most 100 characters")]
    TitleTooLong,
    #[error("body must not be empty")]
    EmptyBody,
    #[error("body must contain at most 2000 characters")]
    BodyTooLong,
    #[error("rating must be between 1 and 5")]
    InvalidRating,
    #[error("at least one field must be provided")]
    NoChanges,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl CreateFeedback {
    /// Валидирует и нормализует DTO, возвращая отдельный проверенный тип.
    pub fn validate(self) -> Result<ValidatedFeedback, ValidationError> {
        todo!("проверить title, body, rating и ограничения chars().count()")
    }
}

impl UpdateFeedback {
    /// Требует хотя бы одно Some-поле и нормализует присутствующие значения.
    pub fn validate(self) -> Result<Self, ValidationError> {
        todo!("проверить частичное обновление")
    }
}

/// Создаёт DTO ответа 201 или ошибку 422 с Display-текстом ValidationError.
pub async fn create_feedback(
    Json(input): Json<CreateFeedback>,
) -> Result<(StatusCode, Json<ValidatedFeedback>), (StatusCode, Json<ErrorResponse>)> {
    todo!("применить validate и сопоставить результат с HTTP")
}

/// Регистрирует POST /feedback.
pub fn app() -> Router {
    todo!("создать router для create_feedback")
}
