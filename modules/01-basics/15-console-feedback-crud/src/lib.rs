#![allow(unused_variables, dead_code)]

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FeedbackStatus {
    New,
    InReview,
    Resolved,
    Rejected,
}

impl FeedbackStatus {
    /// Тот же workflow, что в уроке 06: same, New→InReview/Rejected,
    /// InReview→Resolved/Rejected; остальные переходы запрещены.
    pub fn can_transition_to(self, next: Self) -> bool {
        todo!("реализовать машину состояний")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub body: String,
    pub rating: u8,
    pub status: FeedbackStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewFeedback {
    pub title: String,
    pub body: String,
    pub rating: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct UpdateFeedback {
    pub title: Option<String>,
    pub body: Option<String>,
    pub rating: Option<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ServiceError {
    #[error("title must not be empty")]
    InvalidTitle,
    #[error("body must not be empty")]
    InvalidBody,
    #[error("rating must be between 1 and 5")]
    InvalidRating,
    #[error("feedback not found")]
    NotFound,
    #[error("invalid status transition")]
    InvalidTransition,
    #[error("invalid command: {0}")]
    InvalidCommand(String),
    #[error("storage error: {0}")]
    Storage(String),
}

pub trait FeedbackRepository {
    fn load(&self) -> Result<Vec<Feedback>, ServiceError>;
    fn save(&self, feedback: &[Feedback]) -> Result<(), ServiceError>;
}

#[derive(Debug, Clone)]
pub struct JsonFileRepository {
    path: PathBuf,
}

impl JsonFileRepository {
    pub fn new(path: PathBuf) -> Self {
        todo!("сохранить путь к JSON-файлу")
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl FeedbackRepository for JsonFileRepository {
    /// Отсутствующий файл трактуется как пустое хранилище.
    fn load(&self) -> Result<Vec<Feedback>, ServiceError> {
        todo!("прочитать JSON или вернуть пустой Vec для NotFound")
    }

    fn save(&self, feedback: &[Feedback]) -> Result<(), ServiceError> {
        todo!("записать pretty JSON с ошибками Storage")
    }
}

pub struct FeedbackService<R> {
    repository: R,
}

impl<R: FeedbackRepository> FeedbackService<R> {
    pub fn new(repository: R) -> Self {
        todo!("сохранить repository")
    }

    /// Создаёт запись со статусом New и id = max(existing id) + 1, начиная с 1.
    pub fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError> {
        todo!("загрузить, провалидировать, добавить и сохранить")
    }

    pub fn list(&self) -> Result<Vec<Feedback>, ServiceError> {
        todo!("вернуть записи по возрастанию id")
    }

    pub fn get(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("вернуть клон записи или NotFound")
    }

    pub fn update(&self, id: u64, update: UpdateFeedback) -> Result<Feedback, ServiceError> {
        todo!("применить только Some-поля и сохранить")
    }

    pub fn set_status(&self, id: u64, status: FeedbackStatus) -> Result<Feedback, ServiceError> {
        todo!("проверить переход до изменения записи")
    }

    pub fn delete(&self, id: u64) -> Result<Feedback, ServiceError> {
        todo!("удалить, сохранить остаток и вернуть запись")
    }
}

/// Выполняет CLI-команду и возвращает JSON-результат.
pub fn run_cli(args: &[String], repository: JsonFileRepository) -> Result<String, ServiceError> {
    todo!("разобрать create/list/get/update/status/delete и вызвать service")
}
