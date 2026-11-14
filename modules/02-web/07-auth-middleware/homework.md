# Домашнее задание: JWT extractor

```rust
pub fn validate_token(token: &str, config: &JwtConfig, now: i64)
    -> Result<Claims, AuthError>;
pub fn bearer_token(headers: &HeaderMap) -> Result<&str, AuthError>;
pub fn current_user(claims: &Claims) -> Result<CurrentUser, AuthError>;
pub fn AuthState::new(config: JwtConfig) -> Self;
pub async fn profile(user: CurrentUser) -> Json<CurrentUserResponse>;
pub fn protected_app(state: AuthState) -> Router;
```

Также реализуйте `FromRequestParts<AuthState> for CurrentUser` в заготовке.

## Контракт

- `validate_token`: только HS256, обязательные issuer/audience, валидный формат claims. Отключите автоматическую exp-проверку и используйте параметр now: `now >= exp` → `ExpiredToken`, `iat > now` → `InvalidToken`. Остальные ошибки decode → `InvalidToken`.
- `bearer_token`: отсутствие Authorization → `MissingAuthorization`; не-ASCII, неправильная схема/число слов, пустой token → `InvalidAuthorization`. Схема Bearer case-insensitive, между схемой и token ровно один пробел, пробелы по краям всего значения недопустимы.
- `current_user`: sub разбирается в `u64` и должен быть больше 0, иначе `InvalidSubject`.
- `FromRequestParts`: последовательность headers → bearer → validate с `Utc::now().timestamp()` → current_user. Любой вариант AuthError превращается в `(401 UNAUTHORIZED, Json(ErrorResponse { error: error.to_string() }))`.
- `profile`: копирует id/role в CurrentUserResponse.
- `protected_app`: `GET /profile` и переданный AuthState.

Не добавляйте role checks в этот урок: private moderator routes появятся в уроке 08.

## Проверка

```bash
cargo test -p web-07-auth-middleware
cargo clippy -p web-07-auth-middleware --all-targets -- -D warnings
```
