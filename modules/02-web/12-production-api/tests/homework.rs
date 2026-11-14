use axum::body::Body;
use axum::http::{HeaderMap, Method, Request, StatusCode};
use tower::ServiceExt;
use web_12_production::{ErrorKind, ProblemDetails, app, problem, request_id, status};

#[test]
fn problem_contains_safe_machine_readable_contract() {
    let problem = problem(ErrorKind::Internal, "req-1", "/feedback");
    assert_eq!(problem.status, 500);
    assert_eq!(problem.title, "Internal Server Error");
    assert_eq!(problem.detail, "internal server error");
    assert_eq!(problem.request_id, "req-1");
    assert_eq!(problem.instance, "/feedback");
    assert_eq!(status(ErrorKind::NotFound), StatusCode::NOT_FOUND);
    assert_eq!(
        status(ErrorKind::ServiceUnavailable),
        StatusCode::SERVICE_UNAVAILABLE
    );
}

#[test]
fn request_id_preserves_valid_header_and_generates_missing() {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", "req-123".parse().unwrap());
    assert_eq!(request_id(&headers), "req-123");
    let generated = request_id(&HeaderMap::new());
    assert_eq!(generated.len(), 36);
}

#[tokio::test]
async fn error_route_returns_problem_json_and_propagates_request_id() {
    let request = Request::builder()
        .method(Method::GET)
        .uri("/error")
        .header("x-request-id", "req-abc")
        .body(Body::empty())
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(response.headers().get("x-request-id").unwrap(), "req-abc");
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/problem+json"
    );
    let bytes = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    let problem: ProblemDetails = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(problem.request_id, "req-abc");
    assert_eq!(problem.detail, "internal server error");
}

#[tokio::test]
async fn body_limit_rejects_seventeen_bytes() {
    let small = Request::builder()
        .method(Method::POST)
        .uri("/limited")
        .body(Body::from("1234567890123456"))
        .unwrap();
    assert_eq!(app().oneshot(small).await.unwrap().status(), StatusCode::OK);

    let large = Request::builder()
        .method(Method::POST)
        .uri("/limited")
        .body(Body::from("12345678901234567"))
        .unwrap();
    assert_eq!(
        app().oneshot(large).await.unwrap().status(),
        StatusCode::PAYLOAD_TOO_LARGE
    );
}
