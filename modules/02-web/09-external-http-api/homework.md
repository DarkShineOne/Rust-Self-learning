# Домашнее задание: HttpNotifier

```rust
pub fn HttpNotifier::new(config: NotifierConfig)
    -> Result<Self, NotificationError>;
pub async fn notify(&self, notification: &Notification)
    -> Result<NotificationResponse, NotificationError>;
```

## Конфигурация и запрос

- `new`: base_url обязан разбираться как абсолютный `http://` или `https://` URL с host; api_key непустой после trim. Ошибки → `InvalidConfig(понятное_сообщение)`. Client строится с заданным timeout.
- URL запроса: `base_url.trim_end_matches('/') + "/v1/notifications"`.
- POST JSON Notification, `Authorization: Bearer <api_key>`, `Idempotency-Key: feedback-<feedback_id>`.
- Успешный 2xx разбирается как NotificationResponse. `accepted=false` → `Rejected`, ошибка JSON → `InvalidResponse`.

## Retry и ошибки

- Всего не более `max_retries + 1` попыток.
- Retry разрешён для ошибок timeout/transport при запросе и статусов 502/503/504. Не повторяйте ошибку декодирования JSON или Rejected.
- После последнего retryable status → `ServerStatus(status)`. Любой другой неуспешный status → `UnexpectedStatus(status)` без retry.
- Последний timeout → `Timeout`, другая transport-ошибка после последней попытки → `Transport(message)`.
- Дополнительный backoff в задании не требуется. Idempotency-Key обязателен при каждой попытке.

Не логируйте api_key и не выполняйте запросы к реальному интернет-API в тестах.

## Проверка

```bash
cargo test -p web-09-external-http
cargo clippy -p web-09-external-http --all-targets -- -D warnings
```
