# Домашнее задание: слоистый CRUD

Реализуйте все `todo!()` в `src/lib.rs`.

## Repository

```rust
pub fn MemoryRepository::new() -> Self;
async fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError>;
async fn list(&self) -> Result<Vec<Feedback>, ServiceError>;
async fn get(&self, id: u64) -> Result<Feedback, ServiceError>;
async fn replace(&self, feedback: Feedback) -> Result<Feedback, ServiceError>;
async fn delete(&self, id: u64) -> Result<Feedback, ServiceError>;
```

- ID последовательные от 1; `create` сохраняет DTO с новым id.
- `list` сортирует по id, `get`/`replace`/`delete` отсутствующего id → `NotFound`.
- `replace` не вставляет отсутствующую запись, `delete` возвращает удалённую.
- Возвращаются owned клоны, lock не удерживается после метода.

## Service и состояние

```rust
pub fn FeedbackService::new(repository: Arc<R>) -> Self;
pub async fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError>;
pub async fn list(&self) -> Result<Vec<Feedback>, ServiceError>;
pub async fn get(&self, id: u64) -> Result<Feedback, ServiceError>;
pub async fn update(&self, id: u64, update: UpdateFeedback) -> Result<Feedback, ServiceError>;
pub async fn delete(&self, id: u64) -> Result<Feedback, ServiceError>;
pub fn AppState::new() -> Self;
pub fn status_code(error: &ServiceError) -> StatusCode;
```

- Create/update: title/body сохраняются trimmed и не должны быть пустыми; rating 1–5. Ошибки — соответствующие варианты `ServiceError`.
- Update применяет только `Some`, пустой update возвращает существующую запись. Сначала валидация, затем `replace`.
- `AppState::new` создаёт один MemoryRepository в Arc и один service в Arc.
- `status_code`: `NotFound` → 404, остальные текущие ошибки → 422.

## Handlers и router

Реализуйте пять handlers из заготовки: они только вызывают соответствующий метод service и преобразуют `ServiceError` через `ApiError::from`. Create возвращает 201, остальные — `Json` результата со статусом 200 по умолчанию.

```rust
pub fn app(state: AppState) -> Router;
```

Маршруты:

```text
GET    /api/public/v1/feedback
POST   /api/public/v1/feedback
GET    /api/public/v1/feedback/{id}
PATCH  /api/public/v1/feedback/{id}
DELETE /api/public/v1/feedback/{id}
```

Передайте `state` в Router. Не добавляйте авторизацию раньше соответствующих уроков.

## Проверка

```bash
cargo test -p web-03-architecture
cargo clippy -p web-03-architecture --all-targets -- -D warnings
```
