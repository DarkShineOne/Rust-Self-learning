use sqlx::PgPool;
use uuid::Uuid;
use web_14_final_feedback::domain::*;
use web_14_final_feedback::repository::{
    AuthRepository, FeedbackRepository, PgRepository, RotationResult,
};

#[sqlx::test(migrations = "./migrations")]
async fn auth_repository_rotates_refresh_token_in_transaction(pool: PgPool) {
    let repository = PgRepository::new(pool);
    let user = repository
        .create_user("user@example.com", "argon2-hash")
        .await
        .unwrap();
    assert_eq!(user.role, Role::User);
    assert_eq!(
        repository
            .user_by_email("user@example.com")
            .await
            .unwrap()
            .unwrap()
            .id,
        user.id
    );
    repository
        .insert_refresh(NewRefreshToken {
            token_hash: "old-hash".into(),
            user_id: user.id,
            expires_at: 200,
        })
        .await
        .unwrap();
    let rotation = repository
        .rotate_refresh(
            "old-hash",
            NewRefreshToken {
                token_hash: "new-hash".into(),
                user_id: user.id,
                expires_at: 500,
            },
            100,
        )
        .await
        .unwrap();
    assert_eq!(rotation, RotationResult::Success);
    assert!(
        repository
            .refresh_by_hash("old-hash")
            .await
            .unwrap()
            .unwrap()
            .revoked
    );
    assert!(
        !repository
            .refresh_by_hash("new-hash")
            .await
            .unwrap()
            .unwrap()
            .revoked
    );

    let expired = repository
        .rotate_refresh(
            "new-hash",
            NewRefreshToken {
                token_hash: "must-not-exist".into(),
                user_id: user.id,
                expires_at: 800,
            },
            500,
        )
        .await
        .unwrap();
    assert_eq!(expired, RotationResult::Expired);
    assert!(
        repository
            .refresh_by_hash("must-not-exist")
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn feedback_repository_enforces_scoped_crud(pool: PgPool) {
    let repository = PgRepository::new(pool);
    let owner = repository
        .create_user("owner@example.com", "hash")
        .await
        .unwrap();
    let other = repository
        .create_user("other@example.com", "hash")
        .await
        .unwrap();
    let draft = NewFeedback {
        title: "Title".into(),
        body: "Body".into(),
        rating: 5,
        category: Category::Bug,
    };
    let feedback = repository.create_feedback(owner.id, draft).await.unwrap();
    assert_eq!(feedback.status, FeedbackStatus::New);

    let update = UpdateFeedback {
        title: Some("Updated".into()),
        ..UpdateFeedback::default()
    };
    assert!(
        repository
            .update_owned_feedback(feedback.id, other.id, update.clone())
            .await
            .unwrap()
            .is_none()
    );
    let updated = repository
        .update_owned_feedback(feedback.id, owner.id, update)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.title, "Updated");

    let attachment = Attachment {
        id: Uuid::new_v4(),
        feedback_id: feedback.id,
        key: format!("feedback/{}/{}/note.txt", feedback.id, Uuid::new_v4()),
        original_name: "note.txt".into(),
        content_type: "text/plain".into(),
        size_bytes: 10,
    };
    repository
        .save_attachment(attachment.clone())
        .await
        .unwrap();
    let attachments = repository.list_attachments(feedback.id).await.unwrap();
    assert_eq!(attachments.len(), 1);
    assert_eq!(attachments[0], attachment);
    assert_eq!(
        repository.delete_attachment(attachment.id).await.unwrap(),
        Some(attachment)
    );

    repository
        .update_feedback_status(feedback.id, FeedbackStatus::InReview)
        .await
        .unwrap();
    assert!(
        repository
            .delete_owned_feedback(feedback.id, owner.id)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        repository
            .delete_feedback(feedback.id)
            .await
            .unwrap()
            .is_some()
    );
    assert!(
        repository
            .feedback_by_id(feedback.id)
            .await
            .unwrap()
            .is_none()
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn database_constraints_reject_invalid_rating(pool: PgPool) {
    let repository = PgRepository::new(pool.clone());
    let owner = repository
        .create_user("owner@example.com", "hash")
        .await
        .unwrap();
    let result = repository
        .create_feedback(
            owner.id,
            NewFeedback {
                title: "Invalid".into(),
                body: "Body".into(),
                rating: 6,
                category: Category::Other,
            },
        )
        .await;
    assert!(result.is_err());
    let count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM feedback")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(count, 0);
}
