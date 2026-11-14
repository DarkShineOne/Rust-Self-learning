use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use web_02_validation::{
    Category, CreateFeedback, ErrorResponse, UpdateFeedback, ValidatedFeedback, ValidationError,
    app,
};

fn input() -> CreateFeedback {
    CreateFeedback {
        title: "  Title  ".into(),
        body: "  Body  ".into(),
        rating: 5,
        category: Category::Idea,
    }
}

#[test]
fn validates_and_normalizes_create_dto() {
    let valid = input().validate().unwrap();
    assert_eq!(valid.title, "Title");
    assert_eq!(valid.body, "Body");
    assert_eq!(valid.category, Category::Idea);
}

#[test]
fn validates_character_boundaries_and_rating() {
    let mut value = input();
    value.title = "ё".repeat(100);
    assert!(value.clone().validate().is_ok());
    value.title = "ё".repeat(101);
    assert_eq!(value.validate(), Err(ValidationError::TitleTooLong));

    let mut value = input();
    value.body = " \t ".into();
    assert_eq!(value.validate(), Err(ValidationError::EmptyBody));

    let mut value = input();
    value.rating = 0;
    assert_eq!(value.validate(), Err(ValidationError::InvalidRating));
}

#[test]
fn update_requires_changes_and_preserves_none() {
    assert_eq!(
        UpdateFeedback::default().validate(),
        Err(ValidationError::NoChanges)
    );
    let update = UpdateFeedback {
        title: Some("  Updated  ".into()),
        body: None,
        rating: Some(4),
    }
    .validate()
    .unwrap();
    assert_eq!(update.title.as_deref(), Some("Updated"));
    assert_eq!(update.body, None);
    assert_eq!(update.rating, Some(4));
}

#[tokio::test]
async fn create_route_returns_201_and_normalized_json() {
    let request = Request::builder()
        .method(Method::POST)
        .uri("/feedback")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"title":"  API  ","body":"Body","rating":5,"category":"bug"}"#,
        ))
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: ValidatedFeedback = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body.title, "API");
}

#[tokio::test]
async fn create_route_maps_domain_validation_to_422() {
    let request = Request::builder()
        .method(Method::POST)
        .uri("/feedback")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"title":"","body":"Body","rating":5,"category":"bug"}"#,
        ))
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body: ErrorResponse = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body.error, "title must not be empty");
}
