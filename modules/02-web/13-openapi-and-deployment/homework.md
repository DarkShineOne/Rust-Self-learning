# Домашнее задание: OpenAPI и конфигурация

```rust
pub fn public_openapi() -> utoipa::openapi::OpenApi;
pub fn private_openapi() -> utoipa::openapi::OpenApi;
pub async fn public_docs() -> Json<utoipa::openapi::OpenApi>;
pub async fn private_docs() -> Json<utoipa::openapi::OpenApi>;
pub fn app() -> Router;
pub fn AppConfig::validate(&self) -> Result<(), ConfigError>;
```

## OpenAPI и routes

- Используйте `PublicApiDoc::openapi()` и `PrivateApiDoc::openapi()`.
- Регистрируйте annotated handlers и docs endpoints `/api/public/v1/openapi.json`, `/api/private/v1/openapi.json`.
- Public документ не должен содержать `/api/private/...`, private не должен содержать `/api/public/...`.
- Учебные handler bodies остаются заданием и не требуются для анализа документа.

## Config validation

- `database_url`: непустой и начинается с `postgres://` или `postgresql://`; ошибка `InvalidUrl("database_url")`, пустое значение — `Empty("database_url")`.
- `s3_endpoint` и `notification_url`: каждый должен быть непустым `http://` или `https://` URL с непустым host после схемы; соответствующие Empty/InvalidUrl с именем поля.
- `s3_bucket`, `s3_access_key_id`, `s3_secret_access_key`: непустые после trim, иначе Empty с именем поля.
- `jwt_secret`: минимум 32 байта, иначе `WeakJwtSecret`.
- Проверка в порядке полей структуры, затем notification_url. Не печатайте значения в ошибках.

## Проверка

```bash
cargo test -p web-13-deployment
cargo clippy -p web-13-deployment --all-targets -- -D warnings
```

Dockerfile и Compose являются инфраструктурной заготовкой. S3 устанавливается отдельно и в Compose не добавляется.
