use web_06_authentication::{
    AuthConfig, AuthError, AuthService, ConfigError, Role, hash_password, hash_refresh_token,
    verify_password,
};

fn config() -> AuthConfig {
    AuthConfig::new(
        "s".repeat(32),
        "feedback-service".into(),
        "feedback-api".into(),
        60,
        3600,
    )
    .unwrap()
}

fn service() -> AuthService {
    AuthService::new(config())
}

#[test]
fn config_rejects_weak_secret_and_nonpositive_ttl() {
    assert!(matches!(
        AuthConfig::new("short".into(), "iss".into(), "aud".into(), 60, 60),
        Err(ConfigError::WeakJwtSecret)
    ));
    assert!(matches!(
        AuthConfig::new("s".repeat(32), "iss".into(), "aud".into(), 0, 60),
        Err(ConfigError::InvalidTtl)
    ));
}

#[test]
fn argon2_hash_verifies_only_correct_password() {
    let hash = hash_password("correct horse battery").unwrap();
    assert_ne!(hash, "correct horse battery");
    assert!(verify_password("correct horse battery", &hash).unwrap());
    assert!(!verify_password("wrong password", &hash).unwrap());
}

#[test]
fn refresh_hash_is_deterministic_and_not_plaintext() {
    let hash = hash_refresh_token("random-token");
    assert_eq!(hash.len(), 64);
    assert_eq!(hash, hash_refresh_token("random-token"));
    assert_ne!(hash, "random-token");
}

#[tokio::test]
async fn registration_normalizes_email_and_forbids_role_escalation() {
    let service = service();
    let user = service
        .register(" User@Example.COM ", "long password")
        .await
        .unwrap();
    assert_eq!(user.id, 1);
    assert_eq!(user.email, "user@example.com");
    assert_eq!(user.role, Role::User);
    assert_ne!(user.password_hash, "long password");
    assert_eq!(
        service
            .register("user@example.com", "another password")
            .await,
        Err(AuthError::EmailAlreadyUsed)
    );
    assert_eq!(
        service.register("new@example.com", "short").await,
        Err(AuthError::WeakPassword)
    );
}

#[tokio::test]
async fn login_verifies_credentials_and_issues_tokens() {
    let service = service();
    service
        .register("user@example.com", "long password")
        .await
        .unwrap();
    let tokens = service
        .login(" USER@example.com ", "long password", 1000)
        .await
        .unwrap();
    assert!(!tokens.access_token.is_empty());
    assert!(!tokens.refresh_token.is_empty());
    assert_ne!(tokens.access_token, tokens.refresh_token);
    assert_eq!(tokens.access_expires_at, 1060);
    assert_eq!(
        service
            .login("user@example.com", "wrong password", 1000)
            .await,
        Err(AuthError::InvalidCredentials)
    );
    assert_eq!(
        service
            .login("missing@example.com", "long password", 1000)
            .await,
        Err(AuthError::InvalidCredentials)
    );
}

#[tokio::test]
async fn refresh_rotates_and_rejects_reuse() {
    let service = service();
    service
        .register("user@example.com", "long password")
        .await
        .unwrap();
    let first = service
        .login("user@example.com", "long password", 1000)
        .await
        .unwrap();
    let second = service.refresh(&first.refresh_token, 1001).await.unwrap();
    assert_ne!(first.refresh_token, second.refresh_token);
    assert_eq!(
        service.refresh(&first.refresh_token, 1002).await,
        Err(AuthError::RefreshReused)
    );
    assert!(service.refresh(&second.refresh_token, 1003).await.is_ok());
}

#[tokio::test]
async fn refresh_checks_expiry_and_logout_revokes() {
    let service = service();
    service
        .register("user@example.com", "long password")
        .await
        .unwrap();
    let expired = service
        .login("user@example.com", "long password", 1000)
        .await
        .unwrap();
    assert_eq!(
        service.refresh(&expired.refresh_token, 4600).await,
        Err(AuthError::ExpiredToken)
    );

    let active = service
        .login("user@example.com", "long password", 1000)
        .await
        .unwrap();
    service.logout(&active.refresh_token).await;
    service.logout(&active.refresh_token).await;
    assert_eq!(
        service.refresh(&active.refresh_token, 1001).await,
        Err(AuthError::RefreshReused)
    );
}
