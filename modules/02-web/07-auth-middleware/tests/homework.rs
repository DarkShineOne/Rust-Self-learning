use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode, header::AUTHORIZATION};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use tower::ServiceExt;
use web_07_auth_middleware::{
    AuthError, AuthState, Claims, CurrentUserResponse, JwtConfig, Role, bearer_token, current_user,
    protected_app, validate_token,
};

fn config() -> JwtConfig {
    JwtConfig {
        secret: "s".repeat(32),
        issuer: "feedback-service".into(),
        audience: "feedback-api".into(),
    }
}

fn claims() -> Claims {
    Claims {
        sub: "42".into(),
        role: Role::Moderator,
        iss: "feedback-service".into(),
        aud: "feedback-api".into(),
        exp: 2000,
        iat: 1000,
        jti: "token-id".into(),
    }
}

fn token(config: &JwtConfig, claims: &Claims) -> String {
    encode(
        &Header::new(Algorithm::HS256),
        claims,
        &EncodingKey::from_secret(config.secret.as_bytes()),
    )
    .unwrap()
}

#[test]
fn validates_signature_context_and_time() {
    let config = config();
    let token = token(&config, &claims());
    assert_eq!(validate_token(&token, &config, 1000).unwrap(), claims());
    assert_eq!(
        validate_token(&token, &config, 2000),
        Err(AuthError::ExpiredToken)
    );
    assert_eq!(
        validate_token(&token, &config, 999),
        Err(AuthError::InvalidToken)
    );

    let wrong_issuer = JwtConfig {
        issuer: "other".into(),
        secret: config.secret.clone(),
        audience: config.audience.clone(),
    };
    assert_eq!(
        validate_token(&token, &wrong_issuer, 1000),
        Err(AuthError::InvalidToken)
    );
    assert_eq!(
        validate_token("not-a-jwt", &config, 1000),
        Err(AuthError::InvalidToken)
    );
}

#[test]
fn parses_bearer_and_rejects_malformed_values() {
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, "bEaReR token123".parse().unwrap());
    assert_eq!(bearer_token(&headers), Ok("token123"));
    headers.insert(AUTHORIZATION, "Bearer".parse().unwrap());
    assert_eq!(bearer_token(&headers), Err(AuthError::InvalidAuthorization));
    headers.insert(AUTHORIZATION, "Bearer token extra".parse().unwrap());
    assert_eq!(bearer_token(&headers), Err(AuthError::InvalidAuthorization));
    headers.insert(AUTHORIZATION, "Basic token".parse().unwrap());
    assert_eq!(bearer_token(&headers), Err(AuthError::InvalidAuthorization));
}

#[test]
fn subject_must_be_positive_u64() {
    assert_eq!(current_user(&claims()).unwrap().id, 42);
    let mut invalid = claims();
    invalid.sub = "0".into();
    assert_eq!(current_user(&invalid), Err(AuthError::InvalidSubject));
    invalid.sub = "not-a-number".into();
    assert_eq!(current_user(&invalid), Err(AuthError::InvalidSubject));
}

#[tokio::test]
async fn protected_route_accepts_valid_token_and_rejects_missing_token() {
    let state = AuthState::new(config());
    let token = token(&state.config, &claims());
    let request = Request::builder()
        .method(Method::GET)
        .uri("/profile")
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap();
    let response = protected_app(state.clone()).oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let bytes = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    let user: CurrentUserResponse = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(user.id, 42);
    assert_eq!(user.role, Role::Moderator);

    let request = Request::builder()
        .method(Method::GET)
        .uri("/profile")
        .body(Body::empty())
        .unwrap();
    let response = protected_app(state).oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
