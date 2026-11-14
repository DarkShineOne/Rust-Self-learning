# Домашнее задание: AuthService

Реализуйте все публичные функции/методы и приватный `issue_tokens` с `todo!()`.

## Конфигурация и password hashing

```rust
pub fn AuthConfig::new(...) -> Result<Self, ConfigError>;
pub fn hash_password(password: &str) -> Result<String, AuthError>;
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AuthError>;
pub fn hash_refresh_token(token: &str) -> String;
```

- JWT secret не короче 32 байт, TTL строго положительны. Проверка секрета идёт до TTL.
- Argon2id через `Argon2::default`, случайная соль. Неверный пароль → `Ok(false)`, повреждённый hash → `PasswordHash`.
- Refresh hash — SHA-256 lowercase hex. Хеширование refresh не заменяет Argon2 для паролей.

## AuthService

```rust
pub fn new(config: AuthConfig) -> Self;
pub async fn register(&self, email: &str, password: &str) -> Result<User, AuthError>;
pub async fn login(&self, email: &str, password: &str, now: i64) -> Result<TokenPair, AuthError>;
pub async fn refresh(&self, refresh_token: &str, now: i64) -> Result<TokenPair, AuthError>;
pub async fn logout(&self, refresh_token: &str);
```

- Email: trim + lowercase, обязан содержать ровно один `@`, непустые local/domain и точку в domain после первого символа domain. Примеры валидных: `user@example.com`; невалидных: `a@b`, `@example.com`, `a@@example.com`, `a@example`, пустой.
- Пароль: минимум 12 `.chars().count()`. Регистрация создаёт ID от 1 и только `Role::User`.
- Занятый email → `EmailAlreadyUsed`. Проверьте уникальность снова после дорогого hash, чтобы закрыть race внутри одного service. Не держите lock во время Argon2; используйте `spawn_blocking` для hash/verify в async методах.
- Login: нормализованный email, неверный пароль или отсутствующий email → `InvalidCredentials`.
- `issue_tokens`: JWT HS256 с claims из структуры, `sub = user.id`, `jti = UUID`, `iat = now`, `exp = now + access_ttl`. Refresh — случайная opaque строка достаточной длины; хранить только её hash, user_id, expiry и revoked=false.
- `refresh`: неизвестный hash → `InvalidRefreshToken`; revoked → `RefreshReused`; `now >= expires_at` → `ExpiredToken`. При успехе старый revoked и выпускается новая пара с другим refresh token. Rotation и создание новой записи атомарны относительно одного service.
- `logout`: помечает найденный token revoked; неизвестный/повторный logout не паникует и не возвращает ошибку. Refresh после logout → `RefreshReused`.
- Не возвращайте plaintext refresh из структуры Inner и не логируйте секреты.

## Проверка

```bash
cargo test -p web-06-authentication
cargo clippy -p web-06-authentication --all-targets -- -D warnings
```
