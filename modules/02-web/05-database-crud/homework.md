# Домашнее задание: PgFeedbackRepository

Заполните миграцию по требованиям урока 04 и реализуйте методы:

```rust
pub fn PgFeedbackRepository::new(pool: PgPool) -> Self;
pub async fn create(&self, author_id: i64, draft: NewFeedback)
    -> Result<Feedback, RepositoryError>;
pub async fn get(&self, id: i64) -> Result<Feedback, RepositoryError>;
pub async fn list(&self, filter: FeedbackFilter)
    -> Result<Vec<Feedback>, RepositoryError>;
pub async fn update(&self, id: i64, update: UpdateFeedback)
    -> Result<Feedback, RepositoryError>;
pub async fn delete(&self, id: i64) -> Result<Feedback, RepositoryError>;
pub async fn create_user_and_feedback(
    &self,
    email: &str,
    password_hash: &str,
    draft: NewFeedback,
) -> Result<Feedback, RepositoryError>;
```

## Контракт

- `create`: INSERT со статусом `new`, author и полями draft; вернуть строку через RETURNING.
- `get`: одна строка по id или `NotFound`.
- `list`: сначала проверить `1 <= limit <= 100` и `offset >= 0`, иначе `InvalidPagination`. Фильтры author/status необязательны. ORDER BY id ASC, затем LIMIT/OFFSET. Пустой результат — пустой Vec.
- `update`: меняет только `Some`, updated_at = NOW() при выполненном UPDATE, возвращает изменённую строку. Отсутствие id → `NotFound`. Пустой update может просто вернуть `get(id)`.
- `delete`: удаляет и возвращает строку или `NotFound`.
- `create_user_and_feedback`: в одной транзакции создаёт users с ролью `user`, затем feedback со статусом `new`. При любой ошибке обе операции откатываются.
- Все значения, включая фильтры и pagination, передаются bind-параметрами. SQL fragments динамического запроса должны быть фиксированными.
- `sqlx::Error` распространяется как `RepositoryError::Database`; не заменяйте ошибку соединения на NotFound.

## Проверка

Нужны PostgreSQL и `DATABASE_URL`, как в уроке 04. Миграции применяются через `#[sqlx::test]`.

```bash
cargo test -p web-05-database-crud
cargo clippy -p web-05-database-crud --all-targets -- -D warnings
```

Без БД обязательна хотя бы проверка `cargo test -p web-05-database-crud --no-run`.
