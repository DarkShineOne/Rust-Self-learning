#![allow(unused_variables, dead_code)]

use std::time::Duration;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Notification {
    pub feedback_id: u64,
    pub title: String,
    pub category: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotificationResponse {
    pub accepted: bool,
}

#[derive(Debug, Clone)]
pub struct NotifierConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
    /// Число дополнительных попыток после первого запроса.
    pub max_retries: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum NotificationError {
    #[error("invalid notifier configuration: {0}")]
    InvalidConfig(String),
    #[error("notification request timed out")]
    Timeout,
    #[error("notification transport error: {0}")]
    Transport(String),
    #[error("notification service returned retryable status {0}")]
    ServerStatus(u16),
    #[error("notification service returned status {0}")]
    UnexpectedStatus(u16),
    #[error("notification service rejected the event")]
    Rejected,
    #[error("invalid notification response: {0}")]
    InvalidResponse(String),
}

#[derive(Debug, Clone)]
pub struct HttpNotifier {
    client: reqwest::Client,
    config: NotifierConfig,
}

impl HttpNotifier {
    /// Проверяет http(s) base URL и непустой API key, строит Client с timeout.
    pub fn new(config: NotifierConfig) -> Result<Self, NotificationError> {
        todo!("reqwest::Client::builder и валидация config")
    }

    /// POST {base_url}/v1/notifications с Bearer и Idempotency-Key.
    /// Retry только timeout/connect и 502/503/504.
    pub async fn notify(
        &self,
        notification: &Notification,
    ) -> Result<NotificationResponse, NotificationError> {
        todo!("ограниченный retry, status handling и JSON decode")
    }
}
