use std::time::Duration;

use web_09_external_http::{HttpNotifier, Notification, NotificationError, NotifierConfig};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn notification() -> Notification {
    Notification {
        feedback_id: 42,
        title: "New feedback".into(),
        category: "bug".into(),
    }
}

fn config(server: &MockServer) -> NotifierConfig {
    NotifierConfig {
        base_url: server.uri(),
        api_key: "secret-key".into(),
        timeout: Duration::from_millis(100),
        max_retries: 0,
    }
}

#[tokio::test]
async fn sends_typed_notification_with_auth_and_idempotency_key() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/notifications"))
        .and(header("authorization", "Bearer secret-key"))
        .and(header("idempotency-key", "feedback-42"))
        .and(body_json(notification()))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"accepted": true})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(config(&server)).unwrap();
    let response = notifier.notify(&notification()).await.unwrap();
    assert!(response.accepted);
}

#[tokio::test]
async fn retries_only_retryable_status_up_to_limit() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/v1/notifications"))
        .respond_with(ResponseTemplate::new(503))
        .expect(3)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(NotifierConfig {
        max_retries: 2,
        ..config(&server)
    })
    .unwrap();
    assert_eq!(
        notifier.notify(&notification()).await,
        Err(NotificationError::ServerStatus(503))
    );
}

#[tokio::test]
async fn does_not_retry_unexpected_status() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(500))
        .expect(1)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(NotifierConfig {
        max_retries: 3,
        ..config(&server)
    })
    .unwrap();
    assert_eq!(
        notifier.notify(&notification()).await,
        Err(NotificationError::UnexpectedStatus(500))
    );
}

#[tokio::test]
async fn maps_delay_to_timeout() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(200).set_delay(Duration::from_millis(200)))
        .expect(1)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(NotifierConfig {
        timeout: Duration::from_millis(20),
        ..config(&server)
    })
    .unwrap();
    assert_eq!(
        notifier.notify(&notification()).await,
        Err(NotificationError::Timeout)
    );
}

#[tokio::test]
async fn validates_config_and_response_contract() {
    let error = HttpNotifier::new(NotifierConfig {
        base_url: "file:///tmp/storage".into(),
        api_key: "secret".into(),
        timeout: Duration::from_secs(1),
        max_retries: 0,
    });
    assert!(matches!(error, Err(NotificationError::InvalidConfig(_))));

    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"accepted": false})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(config(&server)).unwrap();
    assert_eq!(
        notifier.notify(&notification()).await,
        Err(NotificationError::Rejected)
    );
}
