use std::sync::Arc;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use web_03_architecture::{
    AppState, Feedback, FeedbackRepository, FeedbackService, MemoryRepository, NewFeedback,
    ServiceError, UpdateFeedback, app,
};

fn draft(rating: u8) -> NewFeedback {
    NewFeedback {
        title: "  Title  ".into(),
        body: "  Body  ".into(),
        rating,
    }
}

fn service() -> FeedbackService<MemoryRepository> {
    FeedbackService::new(Arc::new(MemoryRepository::new()))
}

#[tokio::test]
async fn service_performs_crud_and_normalizes_values() {
    let service = service();
    let created = service.create(draft(5)).await.unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "Title");
    assert_eq!(created.body, "Body");

    let updated = service
        .update(
            1,
            UpdateFeedback {
                title: Some("  Updated  ".into()),
                body: None,
                rating: Some(4),
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated");
    assert_eq!(updated.body, "Body");
    assert_eq!(updated.rating, 4);
    assert_eq!(service.get(1).await.unwrap(), updated);
    let listed = service.list().await.unwrap();
    assert_eq!(listed.len(), 1);
    assert_eq!(listed[0], updated);
    assert_eq!(service.delete(1).await.unwrap(), updated);
    assert_eq!(service.get(1).await, Err(ServiceError::NotFound));
}

#[tokio::test]
async fn invalid_input_does_not_reach_repository() {
    let service = service();
    assert_eq!(
        service.create(draft(0)).await,
        Err(ServiceError::InvalidRating)
    );
    assert!(service.list().await.unwrap().is_empty());
    assert_eq!(service.get(99).await, Err(ServiceError::NotFound));
}

#[tokio::test]
async fn replace_does_not_insert_missing_record() {
    let repository = MemoryRepository::new();
    let missing = Feedback {
        id: 7,
        title: "Missing".into(),
        body: "Body".into(),
        rating: 5,
    };
    assert_eq!(
        repository.replace(missing).await,
        Err(ServiceError::NotFound)
    );
    assert!(repository.list().await.unwrap().is_empty());
}

#[tokio::test]
async fn routes_expose_public_crud_contract() {
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/public/v1/feedback")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"title":"From HTTP","body":"Body","rating":5}"#,
        ))
        .unwrap();
    let response = app(AppState::new()).oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let created: Feedback = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "From HTTP");
}

#[tokio::test]
async fn route_maps_not_found_to_404() {
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/public/v1/feedback/999")
        .body(Body::empty())
        .unwrap();
    let response = app(AppState::new()).oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
