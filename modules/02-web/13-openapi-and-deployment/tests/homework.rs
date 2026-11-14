use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;
use web_13_deployment::{AppConfig, ConfigError, app, private_openapi, public_openapi};

#[test]
fn openapi_documents_do_not_mix_public_and_private_paths() {
    let public = serde_json::to_value(public_openapi()).unwrap();
    let public_paths = public["paths"].as_object().unwrap();
    assert!(public_paths.contains_key("/api/public/v1/feedback"));
    assert!(
        public_paths
            .keys()
            .all(|path| !path.contains("/api/private/"))
    );

    let private = serde_json::to_value(private_openapi()).unwrap();
    let private_paths = private["paths"].as_object().unwrap();
    assert!(private_paths.contains_key("/api/private/v1/feedback/{id}"));
    assert!(
        private_paths
            .keys()
            .all(|path| !path.contains("/api/public/"))
    );
}

#[tokio::test]
async fn public_document_is_available_as_json() {
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/public/v1/openapi.json")
        .body(Body::empty())
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

fn config() -> AppConfig {
    AppConfig {
        database_url: "postgres://localhost/feedback".into(),
        s3_endpoint: "http://127.0.0.1:9000".into(),
        s3_bucket: "feedback".into(),
        s3_access_key_id: "key".into(),
        s3_secret_access_key: "secret".into(),
        jwt_secret: "s".repeat(32),
        notification_url: "https://notifications.example.com".into(),
    }
}

#[test]
fn validates_complete_config_without_leaking_secrets() {
    let config = config();
    assert_eq!(config.validate(), Ok(()));
    let debug = format!("{config:?}");
    assert!(!debug.contains("postgres://localhost/feedback"));
    assert!(!debug.contains(&config.jwt_secret));
    assert!(!debug.contains(&config.s3_secret_access_key));
}

#[test]
fn rejects_invalid_urls_and_weak_secret() {
    let mut value = config();
    value.database_url = "sqlite://feedback".into();
    assert_eq!(
        value.validate(),
        Err(ConfigError::InvalidUrl("database_url"))
    );

    let mut value = config();
    value.s3_bucket = " ".into();
    assert_eq!(value.validate(), Err(ConfigError::Empty("s3_bucket")));

    let mut value = config();
    value.jwt_secret = "short".into();
    assert_eq!(value.validate(), Err(ConfigError::WeakJwtSecret));
}
