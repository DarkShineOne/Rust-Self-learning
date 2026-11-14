#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    DuplicateId,
    NotFound,
    InvalidRating,
    Poisoned,
}

#[derive(Debug, Clone, Default)]
pub struct SharedFeedbackStore {
    inner: Arc<RwLock<HashMap<u64, Feedback>>>,
}

impl SharedFeedbackStore {
    pub fn new() -> Self {
        todo!("создать Arc<RwLock<HashMap<...>>>")
    }

    /// Добавляет уникальную запись.
    pub fn insert(&self, feedback: Feedback) -> Result<(), StoreError> {
        todo!("захватить write lock и проверить duplicate")
    }

    /// Возвращает клон записи, чтобы ссылка не удерживала lock после возврата.
    pub fn get(&self, id: u64) -> Result<Feedback, StoreError> {
        todo!("захватить read lock и клонировать найденную запись")
    }

    /// Обновляет rating 1..=5 без изменения записи при ошибке.
    pub fn update_rating(&self, id: u64, rating: u8) -> Result<(), StoreError> {
        todo!("проверить rating, затем изменить существующую запись")
    }

    pub fn len(&self) -> Result<usize, StoreError> {
        todo!("вернуть размер под read lock")
    }

    pub fn is_empty(&self) -> Result<bool, StoreError> {
        todo!("использовать len и обработать ошибку lock")
    }

    /// Возвращает число сильных Arc-ссылок на общее состояние.
    pub fn strong_count(&self) -> usize {
        todo!("использовать Arc::strong_count")
    }
}
