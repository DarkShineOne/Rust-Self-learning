mod common;

use std::sync::Arc;
use std::time::Duration;

use axum::body::Body;
use axum::http::{Method, Request, StatusCode, header::AUTHORIZATION};
use common::*;
use tower::ServiceExt;
use web_14_final_feedback::api::{app, private_openapi, public_openapi};
use web_14_final_feedback::domain::*;

#[test]
fn roles_and_status_workflow_are_explicit() {
    assert!(Role::Moderator.is_staff());
    assert!(Role::Admin.is_staff());
    assert!(Role::Admin.is_admin());
    assert!(!Role::User.is_staff());
    assert!(!Role::Moderator.is_admin());
    assert!(FeedbackStatus::New.can_transition_to(FeedbackStatus::InReview));
    assert!(FeedbackStatus::InReview.can_transition_to(FeedbackStatus::Resolved));
    assert!(!FeedbackStatus::New.can_transition_to(FeedbackStatus::Resolved));
    assert!(!FeedbackStatus::Resolved.can_transition_to(FeedbackStatus::New));
}

#[tokio::test]
async fn auth_supports_registration_login_rotation_reuse_and_logout() {
    let context = context();
    let user = context
        .state
        .auth
        .register(" User@Example.COM ", "long password")
        .await
        .unwrap();
    assert_eq!(user.email, "user@example.com");
    assert_eq!(user.role, Role::User);

    let first = context
        .state
        .auth
        .login("USER@example.com", "long password", 1000)
        .await
        .unwrap();
    assert_eq!(first.access_expires_at, 1900);
    assert!(
        context
            .repository
            .refresh_tokens
            .lock()
            .unwrap()
            .iter()
            .all(|record| record.token_hash != first.refresh_token)
    );

    let second = context
        .state
        .auth
        .refresh(&first.refresh_token, 1001)
        .await
        .unwrap();
    assert_ne!(first.refresh_token, second.refresh_token);
    assert_eq!(
        context.state.auth.refresh(&first.refresh_token, 1002).await,
        Err(AppError::Unauthorized)
    );
    context
        .state
        .auth
        .logout(&second.refresh_token)
        .await
        .unwrap();
    assert_eq!(
        context
            .state
            .auth
            .refresh(&second.refresh_token, 1003)
            .await,
        Err(AppError::Unauthorized)
    );
}

#[tokio::test]
async fn access_validation_checks_signature_claims_and_subject() {
    let context = context();
    let user = context.repository.seed_user(Role::User, String::new());
    let token = access_token(&user);
    let current = context
        .state
        .auth
        .validate_access(&token, chrono::Utc::now().timestamp())
        .unwrap();
    assert_eq!(current.id, user.id);
    assert_eq!(current.role, Role::User);
    assert_eq!(
        context.state.auth.validate_access("not-a-jwt", 1000),
        Err(AppError::Unauthorized)
    );
}

#[tokio::test]
async fn public_create_persists_and_reports_notification_failure() {
    let context = context();
    let actor = actor(Role::User);
    let created = context
        .state
        .feedback
        .create_public(actor, draft())
        .await
        .unwrap();
    assert_eq!(created.feedback.author_id, actor.id);
    assert_eq!(created.feedback.title, "Title");
    assert_eq!(created.notification_error, None);
    assert_eq!(context.notifier.calls.lock().unwrap().len(), 1);

    let failing = context_with(
        Arc::new(MockRepository::default()),
        Arc::new(MockStorage::default()),
        Arc::new(MockNotifier {
            fail: true,
            ..MockNotifier::default()
        }),
    );
    let outcome = failing
        .state
        .feedback
        .create_public(actor, draft())
        .await
        .unwrap();
    assert!(outcome.notification_error.is_some());
    assert_eq!(failing.repository.feedback.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn public_crud_enforces_owner_and_new_status() {
    let context = context();
    let owner = actor(Role::User);
    let other = actor(Role::User);
    let feedback = context
        .repository
        .seed_feedback(owner.id, FeedbackStatus::New);
    assert_eq!(
        context
            .state
            .feedback
            .get_public(owner, feedback.id)
            .await
            .unwrap(),
        feedback
    );
    assert_eq!(
        context.state.feedback.get_public(other, feedback.id).await,
        Err(AppError::NotFound)
    );
    assert_eq!(
        context
            .state
            .feedback
            .get_public(actor(Role::Moderator), feedback.id)
            .await,
        Err(AppError::Forbidden)
    );

    let updated = context
        .state
        .feedback
        .update_public(
            owner,
            feedback.id,
            UpdateFeedback {
                title: Some("  Updated  ".into()),
                ..UpdateFeedback::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated");
    assert!(
        context
            .state
            .feedback
            .delete_public(owner, feedback.id)
            .await
            .is_ok()
    );

    let reviewed = context
        .repository
        .seed_feedback(owner.id, FeedbackStatus::InReview);
    assert!(
        context
            .state
            .feedback
            .delete_public(owner, reviewed.id)
            .await
            .is_err()
    );
}

#[tokio::test]
async fn private_api_checks_staff_workflow_and_admin_only_delete() {
    let context = context();
    let author = actor(Role::User);
    let feedback = context
        .repository
        .seed_feedback(author.id, FeedbackStatus::New);
    let moderator = actor(Role::Moderator);
    assert_eq!(
        context
            .state
            .feedback
            .get_private(moderator, feedback.id)
            .await
            .unwrap(),
        feedback
    );
    assert_eq!(
        context
            .state
            .feedback
            .get_private(author, feedback.id)
            .await,
        Err(AppError::Forbidden)
    );
    context
        .state
        .feedback
        .moderate_private(moderator, feedback.id, FeedbackStatus::InReview)
        .await
        .unwrap();
    assert_eq!(
        context
            .state
            .feedback
            .moderate_private(moderator, feedback.id, FeedbackStatus::New)
            .await,
        Err(AppError::Conflict("invalid status transition".into()))
    );
    assert_eq!(
        context
            .state
            .feedback
            .delete_private(moderator, feedback.id)
            .await,
        Err(AppError::Forbidden)
    );
    assert!(
        context
            .state
            .feedback
            .delete_private(actor(Role::Admin), feedback.id)
            .await
            .is_ok()
    );
}

#[tokio::test]
async fn attachment_upload_compensates_metadata_failure() {
    let repository = Arc::new(MockRepository {
        fail_attachment_save: true,
        ..MockRepository::default()
    });
    let owner = actor(Role::User);
    let feedback = repository.seed_feedback(owner.id, FeedbackStatus::New);
    let storage = Arc::new(MockStorage::default());
    let context = context_with(
        repository,
        storage.clone(),
        Arc::new(MockNotifier::default()),
    );
    assert_eq!(
        context
            .state
            .feedback
            .upload_attachment_public(owner, feedback.id, upload())
            .await,
        Err(AppError::ServiceUnavailable)
    );
    assert_eq!(storage.deletes.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn attachment_download_and_delete_check_ownership() {
    let context = context();
    let owner = actor(Role::User);
    let other = actor(Role::User);
    let feedback = context
        .repository
        .seed_feedback(owner.id, FeedbackStatus::New);
    let attachment = context
        .state
        .feedback
        .upload_attachment_public(owner, feedback.id, upload())
        .await
        .unwrap();
    let url = context
        .state
        .feedback
        .download_attachment_public(owner, feedback.id, attachment.id, Duration::from_secs(60))
        .await
        .unwrap();
    assert!(url.contains(&attachment.key));
    assert_eq!(
        context
            .state
            .feedback
            .download_attachment_public(other, feedback.id, attachment.id, Duration::from_secs(60))
            .await,
        Err(AppError::NotFound)
    );
    let deleted = context
        .state
        .feedback
        .delete_attachment_public(owner, feedback.id, attachment.id)
        .await
        .unwrap();
    assert_eq!(deleted.attachment, attachment);
    assert_eq!(deleted.cleanup_error, None);
}

fn request(uri: &str, token: Option<String>) -> Request<Body> {
    let builder = Request::builder().method(Method::GET).uri(uri);
    let builder = match token {
        Some(token) => builder.header(AUTHORIZATION, format!("Bearer {token}")),
        None => builder,
    };
    builder.body(Body::empty()).unwrap()
}

#[tokio::test]
async fn private_router_rejects_user_even_without_nginx() {
    let context = context();
    let user = context.repository.seed_user(Role::User, String::new());
    let feedback = context
        .repository
        .seed_feedback(user.id, FeedbackStatus::New);
    let response = app(context.state)
        .oneshot(request(
            &format!("/api/private/v1/feedback/{}", feedback.id),
            Some(access_token(&user)),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    assert_eq!(
        response.headers().get("content-type").unwrap(),
        "application/problem+json"
    );
}

#[tokio::test]
async fn private_router_allows_moderator_and_public_requires_token() {
    let context = context();
    let author = context.repository.seed_user(Role::User, String::new());
    let moderator = context.repository.seed_user(Role::Moderator, String::new());
    let feedback = context
        .repository
        .seed_feedback(author.id, FeedbackStatus::New);
    let response = app(context.state.clone())
        .oneshot(request(
            &format!("/api/private/v1/feedback/{}", feedback.id),
            Some(access_token(&moderator)),
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let response = app(context.state)
        .oneshot(request(
            &format!("/api/public/v1/feedback/{}", feedback.id),
            None,
        ))
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[test]
fn openapi_documents_keep_public_and_private_paths_separate() {
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
