# Домашнее задание: health endpoint

Реализуйте два публичных элемента:

```rust
pub async fn health() -> Json<HealthResponse>;
pub fn app() -> Router;
```

- `health` возвращает `HealthResponse { status: "ok".into() }` через `Json`.
- `app` создаёт Router и регистрирует `GET /health` на этот handler.
- Другие пути остаются незарегистрированными и получают стандартный 404 Axum.
- `src/main.rs` уже запускает приложение на `127.0.0.1:3000`; менять его не требуется.

## Проверка

```bash
cargo test -p web-01-http
cargo clippy -p web-01-http --all-targets -- -D warnings
```

После решения можно дополнительно запустить `cargo run -p web-01-http` и выполнить `curl http://127.0.0.1:3000/health`.
