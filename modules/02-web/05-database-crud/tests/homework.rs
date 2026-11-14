use sqlx::PgPool;
use web_05_database_crud::{
    FeedbackFilter, NewFeedback, PgFeedbackRepository, RepositoryError, UpdateFeedback,
};

fn draft(title: &str, rating: i16) -> NewFeedback {
    NewFeedback {
        title: title.into(),
        body: format!("Body of {title}"),
        rating,
    }
}

async fn create_user(pool: &PgPool, email: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(
        "INSERT INTO users (email, password_hash, role) VALUES ($1, 'hash', 'user') RETURNING id",
    )
    .bind(email)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[sqlx::test(migrations = "./migrations")]
async fn performs_crud_against_real_postgresql(pool: PgPool) {
    let author_id = create_user(&pool, "author@example.com").await;
    let repository = PgFeedbackRepository::new(pool);
    let created = repository
        .create(author_id, draft("First", 5))
        .await
        .unwrap();
    assert_eq!(created.status, "new");
    assert_eq!(repository.get(created.id).await.unwrap(), created);

    let updated = repository
        .update(
            created.id,
            UpdateFeedback {
                title: Some("Updated".into()),
                body: None,
                rating: Some(4),
                status: Some("in_review".into()),
            },
        )
        .await
        .unwrap();
    assert_eq!(updated.title, "Updated");
    assert_eq!(updated.body, created.body);
    assert_eq!(updated.rating, 4);
    assert_eq!(updated.status, "in_review");

    assert_eq!(repository.delete(created.id).await.unwrap(), updated);
    assert!(matches!(
        repository.get(created.id).await,
        Err(RepositoryError::NotFound)
    ));
}

#[sqlx::test(migrations = "./migrations")]
async fn filters_paginates_and_rejects_invalid_pagination(pool: PgPool) {
    let first_author = create_user(&pool, "first@example.com").await;
    let second_author = create_user(&pool, "second@example.com").await;
    let repository = PgFeedbackRepository::new(pool);
    repository
        .create(first_author, draft("One", 5))
        .await
        .unwrap();
    repository
        .create(first_author, draft("Two", 4))
        .await
        .unwrap();
    repository
        .create(second_author, draft("Three", 3))
        .await
        .unwrap();

    let page = repository
        .list(FeedbackFilter {
            author_id: Some(first_author),
            status: Some("new".into()),
            limit: 1,
            offset: 1,
        })
        .await
        .unwrap();
    assert_eq!(page.len(), 1);
    assert_eq!(page[0].title, "Two");

    let error = repository
        .list(FeedbackFilter {
            limit: 0,
            ..FeedbackFilter::default()
        })
        .await;
    assert!(matches!(error, Err(RepositoryError::InvalidPagination)));
}

#[sqlx::test(migrations = "./migrations")]
async fn duplicate_email_rolls_back_whole_transaction(pool: PgPool) {
    create_user(&pool, "duplicate@example.com").await;
    let repository = PgFeedbackRepository::new(pool.clone());
    let result = repository
        .create_user_and_feedback("duplicate@example.com", "new hash", draft("Orphan", 5))
        .await;
    assert!(result.is_err());
    let feedback_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM feedback")
        .fetch_one(&pool)
        .await
        .unwrap();
    let user_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(feedback_count, 0);
    assert_eq!(user_count, 1);
}
