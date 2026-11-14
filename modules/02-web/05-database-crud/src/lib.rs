#![allow(unused_variables, dead_code)]

use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Feedback {
    pub id: i64,
    pub author_id: i64,
    pub title: String,
    pub body: String,
    pub rating: i16,
    pub status: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFeedback {
    pub title: String,
    pub body: String,
    pub rating: i16,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UpdateFeedback {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<i16>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeedbackFilter {
    pub author_id: Option<i64>,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

impl Default for FeedbackFilter {
    fn default() -> Self {
        Self {
            author_id: None,
            status: None,
            limit: 20,
            offset: 0,
        }
    }
}

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("feedback not found")]
    NotFound,
    #[error("invalid pagination")]
    InvalidPagination,
    #[error(transparent)]
    Database(#[from] sqlx::Error),
}

#[derive(Debug, Clone)]
pub struct PgFeedbackRepository {
    pool: PgPool,
}

impl PgFeedbackRepository {
    pub fn new(pool: PgPool) -> Self {
        todo!("сохранить pool")
    }

    /// Создаёт обращение со статусом new.
    pub async fn create(
        &self,
        author_id: i64,
        draft: NewFeedback,
    ) -> Result<Feedback, RepositoryError> {
        todo!("INSERT ... RETURNING с параметрами")
    }

    pub async fn get(&self, id: i64) -> Result<Feedback, RepositoryError> {
        todo!("fetch_optional и NotFound")
    }

    /// Фильтрует, сортирует по id ASC и применяет LIMIT/OFFSET.
    pub async fn list(&self, filter: FeedbackFilter) -> Result<Vec<Feedback>, RepositoryError> {
        todo!("безопасно построить параметризованный запрос")
    }

    /// Применяет только Some-поля и обновляет updated_at = NOW().
    pub async fn update(
        &self,
        id: i64,
        update: UpdateFeedback,
    ) -> Result<Feedback, RepositoryError> {
        todo!("UPDATE существующей записи или NotFound")
    }

    pub async fn delete(&self, id: i64) -> Result<Feedback, RepositoryError> {
        todo!("DELETE ... RETURNING или NotFound")
    }

    /// Создаёт пользователя роли user и его первое обращение в одной транзакции.
    pub async fn create_user_and_feedback(
        &self,
        email: &str,
        password_hash: &str,
        draft: NewFeedback,
    ) -> Result<Feedback, RepositoryError> {
        todo!("BEGIN, два INSERT, COMMIT; rollback при любой ошибке")
    }
}
