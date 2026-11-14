#![allow(unused_variables, dead_code)]

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: u64,
    pub email: String,
    pub password_hash: String,
    pub role: Role,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expires_at: i64,
}

pub struct AuthConfig {
    pub jwt_secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum ConfigError {
    #[error("JWT secret must contain at least 32 bytes")]
    WeakJwtSecret,
    #[error("token TTL must be positive")]
    InvalidTtl,
}

impl AuthConfig {
    pub fn new(
        jwt_secret: String,
        issuer: String,
        audience: String,
        access_ttl_seconds: i64,
        refresh_ttl_seconds: i64,
    ) -> Result<Self, ConfigError> {
        todo!("проверить секрет в байтах и оба TTL")
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum AuthError {
    #[error("invalid email")]
    InvalidEmail,
    #[error("password must contain at least 12 characters")]
    WeakPassword,
    #[error("email is already registered")]
    EmailAlreadyUsed,
    #[error("invalid credentials")]
    InvalidCredentials,
    #[error("invalid refresh token")]
    InvalidRefreshToken,
    #[error("refresh token was already rotated or revoked")]
    RefreshReused,
    #[error("token expired")]
    ExpiredToken,
    #[error("password hashing error: {0}")]
    PasswordHash(String),
    #[error("token encoding error: {0}")]
    TokenEncoding(String),
}

/// Хеширует пароль через Argon2id с случайной солью.
pub fn hash_password(password: &str) -> Result<String, AuthError> {
    todo!("Argon2::default + SaltString::generate + hash_password")
}

/// Проверяет пароль против PHC-строки; неверный пароль возвращает Ok(false).
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AuthError> {
    todo!("PasswordHash::new + verify_password")
}

/// Возвращает нижние 64 hex-символа SHA-256, не храня исходный refresh token.
pub fn hash_refresh_token(token: &str) -> String {
    todo!("SHA-256 для lookup refresh token")
}

#[derive(Debug, Clone)]
struct RefreshRecord {
    user_id: u64,
    expires_at: i64,
    revoked: bool,
}

#[derive(Debug, Default)]
struct Inner {
    users: HashMap<u64, User>,
    emails: HashMap<String, u64>,
    refresh_tokens: HashMap<String, RefreshRecord>,
    next_user_id: u64,
}

#[derive(Debug)]
pub struct AuthService {
    config: AuthConfig,
    inner: RwLock<Inner>,
}

impl std::fmt::Debug for AuthConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("AuthConfig")
            .field("jwt_secret", &"[REDACTED]")
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("access_ttl_seconds", &self.access_ttl_seconds)
            .field("refresh_ttl_seconds", &self.refresh_ttl_seconds)
            .finish()
    }
}

impl AuthService {
    pub fn new(config: AuthConfig) -> Self {
        todo!("инициализировать next_user_id = 1 и пустое состояние")
    }

    /// Регистрирует только роль User. Email нормализуется trim + lowercase.
    pub async fn register(&self, email: &str, password: &str) -> Result<User, AuthError> {
        todo!("валидация, Argon2id вне lock, повторная проверка уникальности")
    }

    /// Проверяет пароль и выпускает access/refresh пару.
    pub async fn login(
        &self,
        email: &str,
        password: &str,
        now: i64,
    ) -> Result<TokenPair, AuthError> {
        todo!("найти пользователя, verify_password, issue_tokens")
    }

    /// Атомарно отзывает старый refresh и выпускает новую пару.
    pub async fn refresh(&self, refresh_token: &str, now: i64) -> Result<TokenPair, AuthError> {
        todo!("проверить hash/revoked/expiry и выполнить rotation")
    }

    /// Отзывает refresh; повторный и неизвестный logout идемпотентно успешны.
    pub async fn logout(&self, refresh_token: &str) {
        todo!("пометить найденный token revoked")
    }

    async fn issue_tokens(&self, user: &User, now: i64) -> Result<TokenPair, AuthError> {
        todo!("JWT HS256 + opaque UUID refresh + SHA-256 storage")
    }
}
