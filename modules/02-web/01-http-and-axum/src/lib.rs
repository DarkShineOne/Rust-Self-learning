#![allow(unused_variables, dead_code)]

use axum::{Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// Возвращает JSON {"status":"ok"}.
pub async fn health() -> Json<HealthResponse> {
    todo!("вернуть HealthResponse")
}

/// Собирает Router с GET /health.
pub fn app() -> Router {
    todo!("зарегистрировать health handler")
}
