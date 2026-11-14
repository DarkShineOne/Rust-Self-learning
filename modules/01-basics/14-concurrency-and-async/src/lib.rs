#![allow(unused_variables, dead_code)]

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::{RwLock, mpsc};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Feedback {
    pub id: u64,
    pub rating: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StoreError {
    DuplicateId,
}

#[derive(Debug, Clone, Default)]
pub struct SharedStore {
    inner: Arc<RwLock<HashMap<u64, Feedback>>>,
}

impl SharedStore {
    pub fn new() -> Self {
        todo!("создать асинхронное разделяемое состояние")
    }

    pub async fn put(&self, feedback: Feedback) -> Result<(), StoreError> {
        todo!("использовать write().await и проверить duplicate")
    }

    pub async fn get(&self, id: u64) -> Option<Feedback> {
        todo!("клонировать запись под read lock")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReceiveError {
    Timeout,
    Closed,
}

/// Ждёт следующее сообщение не дольше timeout.
pub async fn receive_with_timeout(
    receiver: &mut mpsc::Receiver<Feedback>,
    timeout: Duration,
) -> Result<Feedback, ReceiveError> {
    todo!("обернуть receiver.recv в tokio::time::timeout")
}

/// Читает все записи из независимых store и возвращает отсортированные оценки.
pub async fn collect_ratings(stores: Vec<SharedStore>) -> Vec<u8> {
    todo!("обработать независимые store конкурентно и отсортировать результат")
}
