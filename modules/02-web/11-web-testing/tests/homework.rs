use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;
use web_11_testing::{
    AppState, CreationOutcome, Feedback, FeedbackRepository, FeedbackService, NewFeedback,
    Notification, Notifier, ServiceError, app,
};

#[derive(Default)]
struct MockRepository {
    saved: Mutex<Vec<NewFeedback>>,
    fail: bool,
}

#[async_trait]
impl FeedbackRepository for MockRepository {
    async fn insert(&self, draft: NewFeedback) -> Result<Feedback, ServiceError> {
        if self.fail {
            return Err(ServiceError::Repository("database unavailable".into()));
        }
        self.saved.lock().unwrap().push(draft.clone());
        Ok(Feedback {
            id: 1,
            title: draft.title,
            rating: draft.rating,
        })
    }
}

#[derive(Default)]
struct MockNotifier {
    calls: Mutex<Vec<Notification>>,
    fail: bool,
}

#[async_trait]
impl Notifier for MockNotifier {
    async fn notify(&self, notification: Notification) -> Result<(), String> {
        self.calls.lock().unwrap().push(notification);
        if self.fail {
            Err("notifier unavailable".into())
        } else {
            Ok(())
        }
    }
}

fn draft() -> NewFeedback {
    NewFeedback {
        title: "  Title  ".into(),
        rating: 5,
    }
}

#[tokio::test]
async fn valid_create_persists_then_notifies() {
    let repository = Arc::new(MockRepository::default());
    let notifier = Arc::new(MockNotifier::default());
    let service = FeedbackService::new(repository.clone(), notifier.clone());
    let outcome = service.create(draft()).await.unwrap();
    assert_eq!(outcome.feedback.title, "Title");
    assert_eq!(outcome.notification_error, None);
    assert_eq!(repository.saved.lock().unwrap().len(), 1);
    assert_eq!(notifier.calls.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn invalid_input_has_no_side_effects() {
    let repository = Arc::new(MockRepository::default());
    let notifier = Arc::new(MockNotifier::default());
    let service = FeedbackService::new(repository.clone(), notifier.clone());
    let mut invalid = draft();
    invalid.rating = 0;
    assert_eq!(
        service.create(invalid).await,
        Err(ServiceError::InvalidRating)
    );
    assert!(repository.saved.lock().unwrap().is_empty());
    assert!(notifier.calls.lock().unwrap().is_empty());
}

#[tokio::test]
async fn notification_failure_is_explicit_and_does_not_rollback() {
    let repository = Arc::new(MockRepository::default());
    let notifier = Arc::new(MockNotifier {
        fail: true,
        ..MockNotifier::default()
    });
    let service = FeedbackService::new(repository.clone(), notifier.clone());
    let outcome = service.create(draft()).await.unwrap();
    assert_eq!(
        outcome.notification_error.as_deref(),
        Some("notifier unavailable")
    );
    assert_eq!(repository.saved.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn repository_failure_prevents_notification() {
    let repository = Arc::new(MockRepository {
        fail: true,
        ..MockRepository::default()
    });
    let notifier = Arc::new(MockNotifier::default());
    let service = FeedbackService::new(repository, notifier.clone());
    assert!(service.create(draft()).await.is_err());
    assert!(notifier.calls.lock().unwrap().is_empty());
}

#[tokio::test]
async fn route_exposes_creation_outcome() {
    let repository = Arc::new(MockRepository::default());
    let notifier = Arc::new(MockNotifier::default());
    let service = Arc::new(FeedbackService::new(repository, notifier));
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/public/v1/feedback")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"title":"From route","rating":5}"#))
        .unwrap();
    let response = app(AppState::new(service)).oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    let outcome: CreationOutcome = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(outcome.feedback.title, "From route");
}
