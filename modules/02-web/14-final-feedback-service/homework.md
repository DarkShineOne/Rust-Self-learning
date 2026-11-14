# Итоговое домашнее задание: микросервис обратной связи

Реализуйте все `todo!()` в `src`, миграцию и конфигурацию запуска. Не меняйте публичные типы и тесты. Готового ответа в репозитории нет. Работа принимается после прохождения тестов и ревью.

## 1. Domain

```rust
pub fn Role::is_staff(self) -> bool;
pub fn Role::is_admin(self) -> bool;
pub fn FeedbackStatus::can_transition_to(self, next: Self) -> bool;
```

- Staff: Moderator/Admin. Admin: только Admin.
- Workflow: same разрешён; New→InReview/Rejected; InReview→Resolved/Rejected; остальные переходы запрещены.

`AppError` используется на всех границах. Не добавляйте секретный контекст в варианты и не превращайте NotFound в Internal.

## 2. Auth

```rust
pub fn JwtConfig::validate(&self) -> Result<(), AppError>;
pub fn hash_password(password: &str) -> Result<String, AppError>;
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError>;
pub fn hash_refresh_token(token: &str) -> String;
pub fn AuthService::new(repository: Arc<dyn AuthRepository>, config: JwtConfig)
    -> Result<Self, AppError>;
pub async fn register(&self, email: &str, password: &str)
    -> Result<UserResponse, AppError>;
pub async fn login(&self, email: &str, password: &str, now: i64)
    -> Result<TokenPair, AppError>;
pub async fn refresh(&self, refresh_token: &str, now: i64)
    -> Result<TokenPair, AppError>;
pub async fn logout(&self, refresh_token: &str) -> Result<(), AppError>;
pub fn validate_access(&self, token: &str, now: i64)
    -> Result<CurrentUser, AppError>;
```

- Config: JWT secret не короче 32 байт, issuer/audience непустые, TTL положительные.
- Email: trim + lowercase, один `@`, непустые local/domain и точка в domain после первого символа. Пароль: минимум 12 `.chars().count()`.
- Registration создаёт только User. Занятый email → `Conflict("email is already registered")`. Некорректные значения → `Validation`.
- Argon2id используется для пароля; дорогие операции в async переносятся в `spawn_blocking`. При login отсутствие email и неверный пароль приводят к одинаковой ошибке `Unauthorized`, не раскрывая причину.
- Access JWT: HS256, claims из domain, `token_type=access`, `jti=UUID`, `iat=now`, `exp=now+access_ttl`.
- Refresh: случайная opaque строка, в БД только SHA-256 lowercase hex. Login вставляет запись с `now+refresh_ttl`.
- `refresh`: NotFound/revoked/expired → `Unauthorized`; при успехе атомарная rotation и новая пара. Повтор старого запрещён.
- `logout`: идемпотентный revoke по hash; неизвестный token не вызывает ошибку.
- `validate_access`: алгоритм HS256, issuer/audience, `token_type=access`, `now < exp`, `iat <= now`, sub содержит валидный UUID. Любая ошибка → `Unauthorized`.

## 3. Repository и PostgreSQL

```rust
pub fn PgRepository::new(pool: PgPool) -> Self;
```

Реализуйте все методы `AuthRepository` и `FeedbackRepository` из `src/repository.rs` для `PgRepository` с сохранением сигнатур.

- Все значения через bind. Не используйте пользовательские значения в `format!` SQL.
- `create_user`: role `user`; unique email → Conflict; остальные ошибки БД → ServiceUnavailable с безопасным логированием.
- `insert_refresh`: только hash, user_id, expires_at. `revoke_refresh` идемпотентен.
- `rotate_refresh`: transaction и `SELECT ... FOR UPDATE` старого hash. NotFound/Reused/Expired возвращаются соответствующим `RotationResult`. Успех: revoke старого + insert нового + `Success`, один commit.
- Feedback create: UUID, status New, created_at/updated_at NOW, вернуть созданную запись.
- `feedback_by_id`/attachment_by_id: `Ok(None)` при отсутствии, ошибка соединения не превращается в None.
- `list_feedback`: фильтры author/status/category, стабильный ORDER BY created_at DESC, id DESC, LIMIT/OFFSET. `1 <= limit <= 100`, `offset >= 0`, иначе Validation.
- `update_owned_feedback` и `delete_owned_feedback`: только `WHERE id = ... AND author_id = ... AND status = 'new'`. Применяются Some-поля; updated_at меняется при update. Ноль строк → `Ok(None)`.
- `update_feedback_status`: transaction с проверкой текущего статуса и workflow. Недопустимый переход → Conflict.
- `delete_feedback`: transaction удаляет feedback и metadata attachments. Сами объекты S3 удаляет service после получения списка keys.
- Attachment save/list/get/delete используют UUID и FK. `delete_attachment` возвращает удалённые metadata.

### Миграция

Заполните `migrations/0001_init.sql`:

- `users`: `id UUID PK`, `email TEXT UNIQUE NOT NULL`, `password_hash TEXT NOT NULL`, `role TEXT NOT NULL`, `created_at TIMESTAMPTZ NOT NULL`;
- `refresh_tokens`: `token_hash TEXT PK`, `user_id UUID NOT NULL FK users ON DELETE CASCADE`, `expires_at BIGINT NOT NULL`, `revoked BOOLEAN NOT NULL DEFAULT FALSE`, `created_at TIMESTAMPTZ NOT NULL`, index user_id;
- `feedback`: `id UUID PK`, `author_id UUID NOT NULL FK users ON DELETE RESTRICT`, `title/body TEXT NOT NULL`, `rating SMALLINT NOT NULL` check 1–5, `category/status TEXT NOT NULL` с checks вариантов, `created_at/updated_at TIMESTAMPTZ NOT NULL`, indexes author/status/category/created_at;
- `attachments`: `id UUID PK`, `feedback_id UUID NOT NULL FK feedback ON DELETE CASCADE`, `key TEXT UNIQUE NOT NULL`, `original_name TEXT NOT NULL`, `content_type TEXT NOT NULL`, `size_bytes BIGINT NOT NULL CHECK >= 0`, `created_at TIMESTAMPTZ NOT NULL`, index feedback_id.

Checks должны перечислять только варианты domain. Не используйте DROP против существующих данных.

## 4. S3

```rust
pub fn validate_upload(upload: &AttachmentUpload) -> Result<(), AppError>;
pub fn S3Storage::new(client: aws_sdk_s3::Client, bucket: String)
    -> Result<Self, AppError>;
pub fn S3Storage::object_key(feedback_id: Uuid, attachment_id: Uuid, file_name: &str)
    -> Result<String, AppError>;
```

И методы `ObjectStorage` для S3Storage:

```rust
async fn put(&self, feedback_id: Uuid, attachment_id: Uuid, upload: AttachmentUpload)
    -> Result<Attachment, AppError>;
async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, AppError>;
async fn delete(&self, key: &str) -> Result<(), AppError>;
```

- Пустой файл → Validation; `max_bytes == 0` или превышение → PayloadTooLarge.
- Имя после trim непустое, его длина — до 255 символов; запрещены `/`, `\\`, control. MIME: text/plain, image/png, image/jpeg, application/pdf.
- Key: `feedback/{feedback_id}/{attachment_id}/{trimmed_name}`. Download/delete принимают только key с prefix `feedback/`, без `..`, backslash/control.
- Put использует ByteStream и возвращает metadata. Presign TTL 1–3600 секунд. Ошибки SDK → ServiceUnavailable/Timeout без секретов.

## 5. Внешний HTTP API

```rust
pub fn HttpNotifier::new(config: NotifierConfig) -> Result<Self, AppError>;
async fn notify(&self, notification: Notification) -> Result<(), AppError>;
```

- POST `base_url.trim_end_matches('/') + "/v1/notifications"`, JSON Notification, Bearer api_key, `Idempotency-Key: feedback-{feedback_id}`.
- Timeout обязателен, не более `max_retries + 1` попыток. Retry только timeout/transport и 502/503/504.
- 2xx с `{"accepted":true}` — успех. accepted=false → ServiceUnavailable. Другие статусы — ServiceUnavailable, timeout — Timeout.
- URL и api_key проверяются при new. Секреты не логируются.

## 6. FeedbackService

```rust
pub fn new(
    repository: Arc<dyn FeedbackRepository>,
    storage: Arc<dyn ObjectStorage>,
    notifier: Arc<dyn Notifier>,
) -> Self;
```

Все методы должны сохранить сигнатуры из `src/service.rs`.

- `create_public`: только User; title/body после trim непустые (до 100/2000 символов), rating 1–5. Create в БД, затем notify. Ошибка notify не откатывает feedback, а помещается в `notification_error` без секретов.
- `get_public`: только User и только своё; чужое → NotFound. `list_public` принудительно устанавливает author_id=actor.id, игнорируя query author_id, и проверяет pagination.
- `update_public`: User/owner/New; validate Some-полей; вызвать scoped update. Ноль строк → NotFound. Status и author не меняются.
- `delete_public`: User/owner/New. Получить metadata attachments, scoped delete, затем best-effort S3 delete каждого key. Вернуть DeleteOutcome с безопасными cleanup errors.
- `list_private`/`get_private`: staff. `moderate_private`: staff и workflow, нарушение → Conflict. `delete_private`: только Admin и тот же cleanup outcome.
- `upload_attachment_public`: User/owner/New, validate, UUID attachment, S3 put, save metadata. При ошибке save один compensating S3 delete, затем исходная ошибка.
- `list_attachments_public` и `download_attachment_public`: User/owner; attachment должен относиться к feedback_id, иначе NotFound. Download получает TTL 60 секунд из handler.
- `delete_attachment_public`: User/owner/New, attachment относится к feedback; удалить metadata, затем S3. Вернуть AttachmentDeleteOutcome с cleanup_error при сбое.
- Private attachment list/download: staff; private delete: только Admin. Проверять соответствие attachment/feedback и возвращать тот же cleanup outcome.
- Проверки роли/владельца выполняются до побочных эффектов. Не удерживайте DB lock во время S3/HTTP.

## 7. API, Problem Details и routers

```rust
pub fn AppState::new(auth: Arc<AuthService>, feedback: Arc<FeedbackService>) -> Self;
pub fn status_for_error(error: &AppError) -> StatusCode;
pub fn ApiError::new(error: AppError, headers: &HeaderMap, instance: &str) -> Self;
pub fn request_id(headers: &HeaderMap) -> String;
pub fn public_openapi() -> utoipa::openapi::OpenApi;
pub fn private_openapi() -> utoipa::openapi::OpenApi;
pub fn public_router(state: AppState) -> Router;
pub fn private_router(state: AppState) -> Router;
pub fn app(state: AppState) -> Router;
pub async fn run() -> Result<(), AppError>;
```

Реализуйте handlers по сигнатурам `src/api.rs` и маршрутам `readme.md`. Handler только извлекает вход, вызывает auth/service и формирует статус/JSON. Create/upload — 201, logout — 204, download — DownloadResponse.

`CurrentUser::from_request_parts`: схема Bearer case-insensitive, ровно один непустой token; validate_access с текущим Utc::now. Отсутствие/невалидность → 401 Problem Details.

- Status: Validation 400, Unauthorized 401, Forbidden 403, NotFound 404, Conflict 409, PayloadTooLarge 413, Timeout 504, ServiceUnavailable 503, Internal 500.
- `ApiError::new`: формат `application/problem+json`, безопасный detail из AppError, request ID из headers или новый UUID, instance из аргумента.
- `public_router`: только public prefix, auth routes и public docs; `private_router`: private prefix, private docs только staff. Не монтируйте private handlers в public router.
- `app`: merge routers, `/health/live`, `/health/ready`, request ID propagation, tracing, JSON/multipart limits и разумный CORS. Readiness не раскрывает ошибки.
- `public_openapi`/`private_openapi`: вызвать соответствующий derive `OpenApi`; пути с разными prefix не смешиваются. Реализуйте docs handlers.
- `run`: `AppConfig::from_env/validate`, tracing, PostgreSQL pool/migrations, AWS SDK custom endpoint/path-style, Reqwest notifier, services, TCP listener и graceful shutdown.

`AppConfig::from_env` читает имена `.env.example`. Обязательные секреты не имеют значений по умолчанию. Значения по умолчанию: bind `127.0.0.1:3000`, DB connections 5, access TTL 900, refresh TTL 2592000, region us-east-1, path-style true, notification timeout 1000 ms/retries 2, attachment limit 5242880. `validate` отклоняет пустые/некорректные значения без печати секретов.

## 8. Проверка и критерии готовности

```bash
cargo test -p web-14-final-feedback --no-run
cargo test -p web-14-final-feedback --test acceptance
cargo test -p web-14-final-feedback --test http_integration
cargo test -p web-14-final-feedback --test database
cargo test -p web-14-final-feedback --test s3_integration -- --ignored
cargo clippy -p web-14-final-feedback --all-targets -- -D warnings
cargo fmt --check
```

- `database` требует DATABASE_URL и права на создание тестовых БД.
- `s3_integration` требует S3_TEST_ENDPOINT/REGION/BUCKET/ACCESS_KEY_ID/SECRET_ACCESS_KEY и заранее созданный test bucket.
- HTTP integration использует Wiremock и не требует внешней сети.
- Acceptance tests используют mocks и не требуют PostgreSQL/S3.
- Не удаляйте и не ослабляйте тесты, не помечайте их ignored ради прохождения.

Готово, когда контракты readme/homework выполнены, все применимые тесты зелёные, Clippy/Rustfmt чистые, а ревью подтверждает безопасность, разделение public/private и согласованность внешних операций.
