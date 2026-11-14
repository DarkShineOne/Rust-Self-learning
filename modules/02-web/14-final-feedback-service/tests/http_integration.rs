use std::time::Duration;

use uuid::Uuid;
use web_14_final_feedback::domain::{Category, Notification};
use web_14_final_feedback::external::{HttpNotifier, Notifier, NotifierConfig};
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn notifier_sends_contract_and_limits_retry() {
    let server = MockServer::start().await;
    let notification = Notification {
        feedback_id: Uuid::new_v4(),
        title: "New feedback".into(),
        category: Category::Bug,
    };
    Mock::given(method("POST"))
        .and(path("/v1/notifications"))
        .and(header("authorization", "Bearer secret"))
        .and(header(
            "idempotency-key",
            format!("feedback-{}", notification.feedback_id).as_str(),
        ))
        .and(body_json(serde_json::to_value(&notification).unwrap()))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"accepted":true})),
        )
        .expect(1)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(NotifierConfig {
        base_url: server.uri(),
        api_key: "secret".into(),
        timeout: Duration::from_millis(100),
        max_retries: 0,
    })
    .unwrap();
    assert_eq!(notifier.notify(notification).await, Ok(()));
}

#[tokio::test]
async fn notifier_retries_503_exactly_max_retries_plus_one() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .respond_with(ResponseTemplate::new(503))
        .expect(3)
        .mount(&server)
        .await;
    let notifier = HttpNotifier::new(NotifierConfig {
        base_url: server.uri(),
        api_key: "secret".into(),
        timeout: Duration::from_millis(100),
        max_retries: 2,
    })
    .unwrap();
    let notification = Notification {
        feedback_id: Uuid::new_v4(),
        title: "Retry".into(),
        category: Category::Other,
    };
    assert!(notifier.notify(notification).await.is_err());
}
