# Домашнее задание: Problem Details и middleware

```rust
pub fn status(kind: ErrorKind) -> StatusCode;
pub fn problem(kind: ErrorKind, request_id: &str, instance: &str) -> ProblemDetails;
pub fn request_id(headers: &HeaderMap) -> String;
pub async fn internal_error(headers: HeaderMap) -> Response;
pub async fn echo(body: String) -> String;
pub fn app() -> Router;
```

## Problem Details

Маппинг:

| Kind | Status | title | detail |
|---|---|---|---|
| Validation | 400 | Bad Request | request validation failed |
| Unauthorized | 401 | Unauthorized | authentication required |
| Forbidden | 403 | Forbidden | access denied |
| NotFound | 404 | Not Found | resource not found |
| Conflict | 409 | Conflict | resource conflict |
| PayloadTooLarge | 413 | Payload Too Large | request body too large |
| Timeout | 504 | Gateway Timeout | upstream service timed out |
| Internal | 500 | Internal Server Error | internal server error |
| ServiceUnavailable | 503 | Service Unavailable | service temporarily unavailable |

`problem_type`: `https://feedback.example.com/problems/` + kebab-case kind (`validation`, `unauthorized`, `forbidden`, `not-found`, `conflict`, `payload-too-large`, `timeout`, `internal`, `service-unavailable`). `instance` и `request_id` копируются из аргументов.

Готовый IntoResponse выставляет `application/problem+json`.

## Request ID и routes

- `request_id`: использовать `x-request-id`, если его значение ASCII и непустое после trim; вернуть исходное значение без trim. Иначе `Uuid::new_v4().to_string()`.
- `internal_error`: Problem Internal с вычисленным request_id и instance `/error`.
- `echo`: вернуть входной String.
- `app`: `GET /error`, `POST /limited`, `DefaultBodyLimit::max(16)`, TraceLayer, SetRequestIdLayer с MakeRequestUuid и PropagateRequestIdLayer для `x-request-id`.
- CORS: origin только `https://feedback.example.com`, methods GET/POST, headers Content-Type/Authorization.
- Порядок layers должен обеспечить request ID в handler и response; тесты проверяют оба.

Не передавайте внутренний текст ошибки в 500. Не логируйте Authorization или тело.

## Проверка

```bash
cargo test -p web-12-production
cargo clippy -p web-12-production --all-targets -- -D warnings
```
