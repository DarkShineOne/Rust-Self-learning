# Домашнее задание: подключение и миграция

## Публичные методы

```rust
pub fn DatabaseConfig::new(url: String, max_connections: u32) -> Result<Self, ConfigError>;
pub fn DatabaseConfig::from_env() -> Result<Self, ConfigError>;
pub async fn connect(config: &DatabaseConfig) -> Result<PgPool, sqlx::Error>;
pub async fn healthcheck(pool: &PgPool) -> Result<i64, sqlx::Error>;
pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::migrate::MigrateError>;
```

- `new`: trimmed URL непустой, max_connections больше 0; хранить trimmed URL. Ошибки `EmptyDatabaseUrl`/`InvalidMaxConnections`.
- `from_env`: отсутствующий `DATABASE_URL` → `MissingDatabaseUrl`; `DB_MAX_CONNECTIONS` по умолчанию `"5"`, ошибка parse → `InvalidMaxConnectionsText(исходное_значение)`. Далее использовать `new`.
- `connect`: `PgPoolOptions` с заданным max_connections и `connect(&config.url)`.
- `healthcheck`: `SELECT 1::BIGINT`, `fetch_one`, результат 1 при доступной БД.
- `migrate`: применить `sqlx::migrate!()` к pool.

## SQL-миграция

Заполните `migrations/0001_create_users_and_feedback.sql`.

`users`:

- `id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY`;
- `email TEXT NOT NULL UNIQUE`;
- `password_hash TEXT NOT NULL`;
- `role TEXT NOT NULL` со значениями только `user`, `moderator`, `admin`;
- `created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`.

`feedback`:

- `id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY`;
- `author_id BIGINT NOT NULL REFERENCES users(id) ON DELETE RESTRICT`;
- `title TEXT NOT NULL`, `body TEXT NOT NULL`;
- `rating SMALLINT NOT NULL` с check 1–5;
- `status TEXT NOT NULL` со значениями `new`, `in_review`, `resolved`, `rejected`;
- `created_at`, `updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()`.

Создайте index `feedback_author_id_idx` по `author_id`. Добавьте ограничениям понятные имена. Не используйте `DROP TABLE` в этой миграции.

## Запуск PostgreSQL

При наличии Docker Compose:

```bash
docker compose up -d postgres
export DATABASE_URL="postgres://feedback:feedback@localhost:5432/feedback"
cargo test -p web-04-postgresql
```

На Windows PowerShell URL задаётся через `$env:DATABASE_URL`. Можно использовать собственный PostgreSQL с правами на создание тестовых БД. Не запускайте тесты против production.

Компиляция без БД:

```bash
cargo test -p web-04-postgresql --no-run
cargo clippy -p web-04-postgresql --all-targets -- -D warnings
```
