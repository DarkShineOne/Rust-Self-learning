#![allow(unused_variables, dead_code, unused_imports)]

use axum::body::Body;
use axum::extract::DefaultBodyLimit;
use axum::http::header::{AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::{Router, routing};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Validation,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    PayloadTooLarge,
    Timeout,
    Internal,
    ServiceUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub problem_type: String,
    pub title: String,
    pub status: u16,
    pub detail: String,
    pub instance: String,
    pub request_id: String,
}

pub fn status(kind: ErrorKind) -> StatusCode {
    todo!("маппинг ErrorKind на HTTP status")
}

/// Создаёт RFC 9457 response только с безопасными статическими detail.
pub fn problem(kind: ErrorKind, request_id: &str, instance: &str) -> ProblemDetails {
    todo!("type/title/status/detail/instance/request_id")
}

impl IntoResponse for ProblemDetails {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (
            status,
            [(
                CONTENT_TYPE,
                HeaderValue::from_static("application/problem+json"),
            )],
            axum::Json(self),
        )
            .into_response()
    }
}

/// Возвращает входной непустой ASCII x-request-id или новый UUID.
pub fn request_id(headers: &HeaderMap) -> String {
    todo!("прочитать x-request-id без паники")
}

/// Возвращает безопасную 500 Problem Details для /error.
pub async fn internal_error(headers: HeaderMap) -> Response {
    todo!("problem(Internal, ...) без внутреннего секретного текста")
}

/// Возвращает текст запроса; Router ограничивает body 16 байтами.
pub async fn echo(body: String) -> String {
    todo!("вернуть body")
}

/// Собирает routes, request ID, tracing, CORS и body limit.
pub fn app() -> Router {
    todo!("GET /error, POST /limited и production layers")
}
