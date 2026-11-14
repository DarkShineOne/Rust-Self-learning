use sqlx::{PgPool, Row};
use web_04_postgresql::{ConfigError, DatabaseConfig, healthcheck};

#[test]
fn validates_explicit_database_config() {
    let config = DatabaseConfig::new("  postgres://localhost/db  ".into(), 5).unwrap();
    assert_eq!(config.url, "postgres://localhost/db");
    assert_eq!(config.max_connections, 5);
    assert_eq!(
        DatabaseConfig::new(" ".into(), 5),
        Err(ConfigError::EmptyDatabaseUrl)
    );
    assert_eq!(
        DatabaseConfig::new("postgres://localhost/db".into(), 0),
        Err(ConfigError::InvalidMaxConnections)
    );
}

#[sqlx::test(migrations = "./migrations")]
async fn healthcheck_uses_real_postgresql(pool: PgPool) {
    assert_eq!(healthcheck(&pool).await.unwrap(), 1);
}

#[sqlx::test(migrations = "./migrations")]
async fn migration_creates_required_tables_and_columns(pool: PgPool) {
    let rows = sqlx::query(
        "SELECT table_name, column_name FROM information_schema.columns \
         WHERE table_schema = 'public' AND table_name IN ('users', 'feedback')",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    let columns: Vec<(String, String)> = rows
        .into_iter()
        .map(|row| (row.get("table_name"), row.get("column_name")))
        .collect();
    for expected in [
        ("users", "id"),
        ("users", "email"),
        ("users", "password_hash"),
        ("users", "role"),
        ("feedback", "author_id"),
        ("feedback", "rating"),
        ("feedback", "status"),
        ("feedback", "updated_at"),
    ] {
        assert!(
            columns
                .iter()
                .any(|column| { column.0 == expected.0 && column.1 == expected.1 }),
            "missing {expected:?}"
        );
    }
}
