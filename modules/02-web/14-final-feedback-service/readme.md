# Итоговый микросервис обратной связи

## Цель

Самостоятельно реализовать микросервис приёма обратной связи по заготовкам и спецификации. В проекте нет эталонного решения. Тесты и ревью проверяют публичный контракт, архитектуру, безопасность и работу с внешними зависимостями.

Перед началом вы должны пройти предыдущие уроки: ownership/traits/async, Axum, PostgreSQL/SQLx, auth, authorization, Reqwest, S3, testing, Problem Details и OpenAPI.

## Архитектура

- `domain.rs` — сущности, DTO, статусы, роли и AppError.
- `auth.rs` — Argon2id, JWT access, opaque rotating refresh, validation.
- `repository.rs` — traits и нерешённый `PgRepository` для PostgreSQL.
- `storage.rs` — `ObjectStorage` и нерешённый S3 адаптер.
- `external.rs` — `Notifier` и нерешённый Reqwest адаптер.
- `service.rs` — доменные сценарии, ownership, роли и согласованность.
- `api.rs` — public/private routers, handlers, extractor, Problem Details и OpenAPI.
- `config.rs` — конфигурация без секретов в Debug.
- `lib.rs` — сборка зависимостей и запуск.

Public/private здесь обозначают группы HTTP endpoints, а не видимость Rust. Публичные методы заготовок нужны тестам и не означают анонимный HTTP-доступ.

## Public API

Префикс `/api/public/v1`:

```text
POST   /auth/register
POST   /auth/login
POST   /auth/refresh
POST   /auth/logout

GET    /feedback
POST   /feedback
GET    /feedback/{id}
PATCH  /feedback/{id}
DELETE /feedback/{id}

POST   /feedback/{feedback_id}/attachments
GET    /feedback/{feedback_id}/attachments
GET    /feedback/{feedback_id}/attachments/{attachment_id}/download
DELETE /feedback/{feedback_id}/attachments/{attachment_id}

GET    /openapi.json
```

Public registration создаёт только `user`. Все feedback/attachment routes требуют access JWT. Пользователь работает только со своими ресурсами. Изменение и удаление feedback и его вложений разрешены только в статусе `new`. Public PATCH не принимает статус или автора.

## Private API

Префикс `/api/private/v1`:

```text
GET    /feedback
GET    /feedback/{id}
PATCH  /feedback/{id}
DELETE /feedback/{id}

GET    /feedback/{feedback_id}/attachments
GET    /feedback/{feedback_id}/attachments/{attachment_id}/download
DELETE /feedback/{feedback_id}/attachments/{attachment_id}

GET    /openapi.json
```

- `moderator` и `admin` читают все записи, фильтруют и модерируют по workflow.
- Только `admin` удаляет feedback и attachments через private API.
- Обычный `user` получает 403 при прямом вызове private router, даже если Nginx отсутствует или настроен неправильно.
- Private OpenAPI также доступен только staff внутри приложения.

## Матрица статусов

- same разрешён;
- `new → in_review | rejected`;
- `in_review → resolved | rejected`;
- `resolved` и `rejected` терминальны для другого статуса.

Пользователь редактирует текст только в `new`. Модератор меняет только status, а не автора и не содержимое.

## Nginx вне задания

Считаем, что внешний Nginx проксирует только `/api/public/v1`. Private prefix доступен только из рабочей сети/VPN или через отдельный внутренний домен. Настройка Nginx, DNS и VPN не входит в задание и не оценивается.

Это defence in depth. Приложение обязано самостоятельно проверять подпись JWT, issuer/audience, срок, тип token, роль и владельца при каждом вызове. Сетевое разделение не заменяет авторизацию.

## Аутентификация

- Пароль хранится только как Argon2id PHC hash.
- Access — короткоживущий JWT HS256 с `sub`, `role`, `iss`, `aud`, `exp`, `iat`, `jti`, `token_type=access`.
- Refresh — случайная opaque строка; в БД только SHA-256 hash.
- Ротация refresh token атомарна в PostgreSQL transaction: заблокировать старую запись, проверить revoked/expiry, отозвать, вставить новую.
- Повторное использование revoked token → 401 и трактуется как reuse. Logout идемпотентен.
- Password hash, access/refresh tokens, JWT secret и credentials не логируются.

## PostgreSQL

Нужны таблицы `users`, `refresh_tokens`, `feedback`, `attachments`. Все идентификаторы — UUID. Feedback хранит автора, title/body/rating/category/status/timestamps. Attachments хранят metadata и S3 key, но не содержимое файла.

Миграции должны содержать PK, FK, unique email, checks rating/status/role/category, индексы для выборок и по refresh hash. Public scoped update/delete используют `WHERE id/author_id/status`, чтобы не потерять защиту при гонке. Ротация refresh token и admin delete используют транзакции.

## S3

Разработчик самостоятельно выбирает и устанавливает open-source S3-совместимый сервер: MinIO, SeaweedFS, Ceph RGW или другой с нужным API. Создайте development/test buckets и ограниченные credentials. S3 намеренно отсутствует в Compose.

Загрузка ограничена по размеру и MIME allowlist. Object key генерируется сервером. Скачивание использует presigned URL TTL 60 секунд. Bucket не является публичным, URL не логируется.

S3 и PostgreSQL не образуют атомарную транзакцию. Upload требует compensating delete при ошибке metadata. Delete возвращает cleanup outcome. Ревью проверяет поведение при сбое и документированную reconciliation-стратегию.

## Внешний HTTP API

Новое обращение вызывает `POST {NOTIFICATION_URL}/v1/notifications` с Bearer и `Idempotency-Key: feedback-{id}`. Client имеет timeout, а retry выполняется только для timeout/transport и 502/503/504, не более `max_retries + 1` попыток.

Обращение остаётся созданным при сбое уведомления. `CreationOutcome.notification_error` содержит безопасную причину. Не выполняйте внешний вызов внутри DB transaction.

## HTTP и production-контракты

- Успешный create: 201, login/refresh/read/update/delete: 200, logout: 204.
- Validation: 400 или 422 — единообразно для каждого слоя; malformed auth: 401; forbidden: 403; hidden/not found: 404; conflict: 409; large file: 413; upstream timeout: 504; unavailable dependency: 503; internal: 500.
- Все ошибки — `application/problem+json` с type/title/status/detail/instance/request_id.
- Request ID присутствует в логах, теле ошибки и response headers.
- Readiness проверяет PostgreSQL и обязательную конфигурацию; liveness не зависит от временного сбоя внешних API.
- OpenAPI public/private разделены. Body/multipart limits, CORS, tracing и graceful shutdown обязательны.

## Тесты

`tests/common/mod.rs` содержит рабочие mocks, а не production-решение. Acceptance tests проверяют public/private authorization, CRUD, refresh rotation, notifications и S3 consistency. `database.rs` требует PostgreSQL, `s3_integration.rs` запускается явно с настоящим bucket, `http_integration.rs` использует Wiremock.

До решения ожидаемы падения на `todo!()`. Сначала добейтесь компиляции, затем проходите тесты по слоям: domain → auth/service mocks → routes → PostgreSQL/S3/HTTP → E2E.

## Критерии ревью

- нет обхода через глобальное состояние или доверенные `X-Role` headers;
- private API защищён даже без Nginx;
- нет plaintext паролей/refresh tokens и секретов в логах;
- SQL параметризован, миграции и транзакции корректны;
- размер файлов ограничен до буферизации, key безопасен;
- согласованность S3/DB и notification partial success явны;
- public/private OpenAPI не смешаны;
- публичные сигнатуры и тесты сохранены, Clippy/Rustfmt проходят.

Подробный контракт каждого метода находится в [homework.md](homework.md).
