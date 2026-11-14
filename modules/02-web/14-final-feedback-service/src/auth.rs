use std::sync::Arc;

use crate::domain::*;
use crate::repository::{AuthRepository, RotationResult};

#[derive(Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub issuer: String,
    pub audience: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

impl std::fmt::Debug for JwtConfig {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("JwtConfig")
            .field("secret", &"[REDACTED]")
            .field("issuer", &self.issuer)
            .field("audience", &self.audience)
            .field("access_ttl_seconds", &self.access_ttl_seconds)
            .field("refresh_ttl_seconds", &self.refresh_ttl_seconds)
            .finish()
    }
}

impl JwtConfig {
    pub fn validate(&self) -> Result<(), AppError> {
        todo!("secret >=32 байт, непустые issuer/audience, положительные TTL")
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    todo!("Argon2id + random salt")
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    todo!("verify PHC string, неверный пароль → Ok(false)")
}

pub fn hash_refresh_token(token: &str) -> String {
    todo!("SHA-256 lowercase hex")
}

pub struct AuthService {
    repository: Arc<dyn AuthRepository>,
    config: JwtConfig,
}

impl AuthService {
    pub fn new(repository: Arc<dyn AuthRepository>, config: JwtConfig) -> Result<Self, AppError> {
        todo!("validate config и сохранить зависимости")
    }

    pub async fn register(&self, email: &str, password: &str) -> Result<UserResponse, AppError> {
        todo!("normalize/validate, password >=12 chars, Argon2, role User")
    }

    pub async fn login(
        &self,
        email: &str,
        password: &str,
        now: i64,
    ) -> Result<TokenPair, AppError> {
        todo!("verify + access JWT + insert opaque refresh")
    }

    pub async fn refresh(&self, refresh_token: &str, now: i64) -> Result<TokenPair, AppError> {
        todo!("atomic rotate_refresh и новая access/refresh пара")
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), AppError> {
        todo!("идемпотентный revoke по hash")
    }

    pub fn validate_access(&self, token: &str, now: i64) -> Result<CurrentUser, AppError> {
        todo!("HS256 + iss/aud + token_type=access + exp/iat + UUID sub")
    }
}
