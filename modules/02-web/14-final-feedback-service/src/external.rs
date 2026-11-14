use std::time::Duration;

use async_trait::async_trait;

use crate::domain::{AppError, Notification};

#[async_trait]
pub trait Notifier: Send + Sync {
    async fn notify(&self, notification: Notification) -> Result<(), AppError>;
}

#[derive(Debug, Clone)]
pub struct NotifierConfig {
    pub base_url: String,
    pub api_key: String,
    pub timeout: Duration,
    pub max_retries: u32,
}

#[derive(Debug, Clone)]
pub struct HttpNotifier {
    client: reqwest::Client,
    config: NotifierConfig,
}

impl HttpNotifier {
    pub fn new(config: NotifierConfig) -> Result<Self, AppError> {
        todo!("проверить URL/api_key и построить Reqwest Client")
    }
}

#[async_trait]
impl Notifier for HttpNotifier {
    async fn notify(&self, notification: Notification) -> Result<(), AppError> {
        todo!("POST /v1/notifications, Bearer, Idempotency-Key, limited retry")
    }
}
