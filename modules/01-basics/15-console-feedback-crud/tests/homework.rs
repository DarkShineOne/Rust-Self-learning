use std::path::{Path, PathBuf};

use basics_15_console::{
    FeedbackRepository, FeedbackService, FeedbackStatus, JsonFileRepository, NewFeedback,
    ServiceError, UpdateFeedback, run_cli,
};
use tempfile::TempDir;

fn fixture() -> (TempDir, PathBuf) {
    let directory = TempDir::new().unwrap();
    let path = directory.path().join("feedback.json");
    (directory, path)
}

fn service(path: &Path) -> FeedbackService<JsonFileRepository> {
    FeedbackService::new(JsonFileRepository::new(path.to_path_buf()))
}

fn draft(rating: u8) -> NewFeedback {
    NewFeedback {
        title: "  Title  ".into(),
        body: "  Body  ".into(),
        rating,
    }
}

#[test]
fn missing_file_is_empty_but_malformed_file_is_an_error() {
    let (_directory, path) = fixture();
    let repository = JsonFileRepository::new(path.clone());
    assert_eq!(repository.load().unwrap(), Vec::new());
    std::fs::write(&path, "not json").unwrap();
    assert!(matches!(repository.load(), Err(ServiceError::Storage(_))));
}

#[test]
fn create_trims_and_persists_feedback() {
    let (_directory, path) = fixture();
    let created = service(&path).create(draft(5)).unwrap();
    assert_eq!(created.id, 1);
    assert_eq!(created.title, "Title");
    assert_eq!(created.body, "Body");
    assert_eq!(created.status, FeedbackStatus::New);

    let reloaded = service(&path).get(1).unwrap();
    assert_eq!(reloaded, created);
}

#[test]
fn invalid_create_does_not_create_file() {
    let (_directory, path) = fixture();
    assert_eq!(
        service(&path).create(draft(0)),
        Err(ServiceError::InvalidRating)
    );
    assert!(!path.exists());
}

#[test]
fn list_update_get_and_delete_follow_crud_contract() {
    let (_directory, path) = fixture();
    let app = service(&path);
    app.create(draft(3)).unwrap();
    app.create(draft(5)).unwrap();
    assert_eq!(app.list().unwrap().len(), 2);

    let updated = app
        .update(
            1,
            UpdateFeedback {
                title: Some("  Updated  ".into()),
                body: None,
                rating: Some(4),
            },
        )
        .unwrap();
    assert_eq!(updated.title, "Updated");
    assert_eq!(updated.body, "Body");
    assert_eq!(updated.rating, 4);
    assert_eq!(app.get(1).unwrap(), updated);

    let deleted = app.delete(1).unwrap();
    assert_eq!(deleted, updated);
    assert_eq!(app.get(1), Err(ServiceError::NotFound));
    assert_eq!(service(&path).list().unwrap().len(), 1);
}

#[test]
fn status_workflow_rejects_invalid_transition_without_persistence() {
    let (_directory, path) = fixture();
    let app = service(&path);
    app.create(draft(5)).unwrap();
    assert_eq!(
        app.set_status(1, FeedbackStatus::Resolved),
        Err(ServiceError::InvalidTransition)
    );
    assert_eq!(app.get(1).unwrap().status, FeedbackStatus::New);

    app.set_status(1, FeedbackStatus::InReview).unwrap();
    app.set_status(1, FeedbackStatus::Resolved).unwrap();
    assert_eq!(
        app.set_status(1, FeedbackStatus::New),
        Err(ServiceError::InvalidTransition)
    );
    assert_eq!(
        service(&path).get(1).unwrap().status,
        FeedbackStatus::Resolved
    );
}

#[test]
fn cli_returns_machine_readable_json() {
    let (_directory, path) = fixture();
    let created = run_cli(
        &[
            "create".into(),
            "CLI title".into(),
            "CLI body".into(),
            "5".into(),
        ],
        JsonFileRepository::new(path.clone()),
    )
    .unwrap();
    let created: serde_json::Value = serde_json::from_str(&created).unwrap();
    assert_eq!(created["id"], 1);
    assert_eq!(created["status"], "new");

    let listed = run_cli(&["list".into()], JsonFileRepository::new(path.clone())).unwrap();
    let listed: serde_json::Value = serde_json::from_str(&listed).unwrap();
    assert_eq!(listed.as_array().unwrap().len(), 1);

    let status = run_cli(
        &["status".into(), "1".into(), "in_review".into()],
        JsonFileRepository::new(path.clone()),
    )
    .unwrap();
    let status: serde_json::Value = serde_json::from_str(&status).unwrap();
    assert_eq!(status["status"], "in_review");
}

#[test]
fn cli_rejects_invalid_syntax_before_service_call() {
    let (_directory, path) = fixture();
    assert_eq!(
        run_cli(&["unknown".into()], JsonFileRepository::new(path.clone())),
        Err(ServiceError::InvalidCommand("unknown".into()))
    );
    assert_eq!(
        run_cli(&["get".into(), "NaN".into()], JsonFileRepository::new(path)),
        Err(ServiceError::InvalidCommand("get NaN".into()))
    );
}
