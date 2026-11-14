# Домашнее задание: проверяемый service и route

```rust
pub fn FeedbackService::new(
    repository: Arc<dyn FeedbackRepository>,
    notifier: Arc<dyn Notifier>,
) -> Self;
pub async fn create(&self, draft: NewFeedback)
    -> Result<CreationOutcome, ServiceError>;
pub fn AppState::new(service: Arc<FeedbackService>) -> Self;
pub async fn create_handler(...)
    -> Result<(StatusCode, Json<CreationOutcome>), ApiError>;
pub fn app(state: AppState) -> Router;
```

## Контракт

- Create: title после trim непустой, rating 1–5. Ошибки `EmptyTitle`/`InvalidRating` до repository.
- Успешный repository.insert вызывается ровно один раз с нормализованным title. После него Notifier вызывается ровно один раз с id/title созданной записи.
- Repository error возвращается без notify.
- Ошибка Notifier не откатывает запись: метод возвращает `Ok(CreationOutcome { feedback, notification_error: Some(message) })`. При успехе notification_error = None.
- Handler возвращает 201 и CreationOutcome; ServiceError преобразуется готовым ApiError.
- Router: `POST /api/public/v1/feedback`.

Тесты содержат mocks и проверяют вызовы. Не меняйте mocks и не добавляйте глобальное состояние.

## Проверка

```bash
cargo test -p web-11-testing
cargo clippy -p web-11-testing --all-targets -- -D warnings
```
