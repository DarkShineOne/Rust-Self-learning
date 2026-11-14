use axum::Extension;
use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;
use web_08_authorization::{
    AppState, AuthzError, CurrentUser, Feedback, FeedbackStatus, Role, authorize_moderation,
    authorize_user_update, authorize_view, private_app, public_app,
};

fn feedback() -> Feedback {
    Feedback {
        id: 7,
        author_id: 42,
        title: "Feedback".into(),
        status: FeedbackStatus::New,
    }
}

fn actor(id: u64, role: Role) -> CurrentUser {
    CurrentUser { id, role }
}

#[test]
fn view_checks_ownership_without_revealing_foreign_resource() {
    assert_eq!(authorize_view(&actor(42, Role::User), &feedback()), Ok(()));
    assert_eq!(
        authorize_view(&actor(1, Role::User), &feedback()),
        Err(AuthzError::NotFound)
    );
    assert_eq!(
        authorize_view(&actor(1, Role::Moderator), &feedback()),
        Ok(())
    );
    assert_eq!(authorize_view(&actor(1, Role::Admin), &feedback()), Ok(()));
}

#[test]
fn public_update_requires_user_owner_and_new_status() {
    assert_eq!(
        authorize_user_update(&actor(42, Role::User), &feedback()),
        Ok(())
    );
    assert_eq!(
        authorize_user_update(&actor(42, Role::Moderator), &feedback()),
        Err(AuthzError::Forbidden)
    );
    assert_eq!(
        authorize_user_update(&actor(1, Role::User), &feedback()),
        Err(AuthzError::NotFound)
    );
    let mut reviewed = feedback();
    reviewed.status = FeedbackStatus::InReview;
    assert_eq!(
        authorize_user_update(&actor(42, Role::User), &reviewed),
        Err(AuthzError::Forbidden)
    );
}

#[test]
fn moderation_checks_role_and_workflow() {
    let moderator = actor(1, Role::Moderator);
    assert_eq!(
        authorize_moderation(&moderator, FeedbackStatus::New, FeedbackStatus::InReview),
        Ok(())
    );
    assert_eq!(
        authorize_moderation(&moderator, FeedbackStatus::New, FeedbackStatus::Resolved),
        Err(AuthzError::InvalidTransition)
    );
    assert_eq!(
        authorize_moderation(
            &actor(42, Role::User),
            FeedbackStatus::New,
            FeedbackStatus::InReview
        ),
        Err(AuthzError::Forbidden)
    );
}

fn request() -> Request<Body> {
    Request::builder()
        .method(Method::GET)
        .uri("/feedback/7")
        .body(Body::empty())
        .unwrap()
}

#[tokio::test]
async fn public_router_allows_owner_and_hides_foreign_feedback() {
    let owner_app = public_app(AppState::new(feedback())).layer(Extension(actor(42, Role::User)));
    assert_eq!(
        owner_app.oneshot(request()).await.unwrap().status(),
        StatusCode::OK
    );

    let foreign_app = public_app(AppState::new(feedback())).layer(Extension(actor(1, Role::User)));
    assert_eq!(
        foreign_app.oneshot(request()).await.unwrap().status(),
        StatusCode::NOT_FOUND
    );

    let moderator_app =
        public_app(AppState::new(feedback())).layer(Extension(actor(1, Role::Moderator)));
    assert_eq!(
        moderator_app.oneshot(request()).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn private_router_rejects_user_even_bypassing_nginx() {
    let user_app = private_app(AppState::new(feedback())).layer(Extension(actor(42, Role::User)));
    assert_eq!(
        user_app.oneshot(request()).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );

    for role in [Role::Moderator, Role::Admin] {
        let staff_app = private_app(AppState::new(feedback())).layer(Extension(actor(1, role)));
        assert_eq!(
            staff_app.oneshot(request()).await.unwrap().status(),
            StatusCode::OK
        );
    }
}
