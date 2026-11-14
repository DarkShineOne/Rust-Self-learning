#![allow(unused_variables, dead_code)]

use std::sync::Arc;

use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderMap, StatusCode};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: Role,
    pub iss: String,
    pub aud: String,
    pub exp: usize,
    pub iat: usize,
    pub jti: String,
}

pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct CurrentUser {
    pub id: u64,
    pub role: Role,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CurrentUserResponse {
    pub id: u64,
    pub role: Role,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthError {
    #[error("missing authorization header")]
    MissingAuthorization,
    #[error("invalid authorization header")]
    InvalidAuthorization,
    #[error("invalid token")]
    InvalidToken,
    #[error("token expired")]
    ExpiredToken,
    #[error("invalid subject")]
    InvalidSubject,
}

/// Проверяет подпись HS256, issuer/audience и iat/exp относительно заданного now.
pub fn validate_token(token: &str, config: &JwtConfig, now: i64) -> Result<Claims, AuthError> {
    todo!("jsonwebtoken decode с явным алгоритмом и ручной проверкой времени")
}

/// Извлекает Bearer token без принятия пустого значения или дополнительных слов.
pub fn bearer_token(headers: &HeaderMap) -> Result<&str, AuthError> {
    todo!("Authorization: Bearer <token>, схема case-insensitive")
}

/// Преобразует положительный numeric sub в CurrentUser.
pub fn current_user(claims: &Claims) -> Result<CurrentUser, AuthError> {
    todo!("разобрать sub и скопировать role")
}

#[derive(Clone)]
pub struct AuthState {
    pub config: Arc<JwtConfig>,
}

impl AuthState {
    pub fn new(config: JwtConfig) -> Self {
        todo!("обернуть config в Arc")
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl FromRequestParts<AuthState> for CurrentUser {
    type Rejection = (StatusCode, Json<ErrorResponse>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AuthState,
    ) -> Result<Self, Self::Rejection> {
        todo!("headers → bearer → validate_token(Utc::now) → current_user; всегда 401")
    }
}

/// Возвращает аутентифицированного пользователя.
pub async fn profile(user: CurrentUser) -> Json<CurrentUserResponse> {
    todo!("скопировать id и role в response")
}

/// Регистрирует GET /profile с AuthState.
pub fn protected_app(state: AuthState) -> Router {
    todo!("Router с CurrentUser extractor")
}
