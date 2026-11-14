# Домашнее задание: DTO и валидация

```rust
impl CreateFeedback {
    pub fn validate(self) -> Result<ValidatedFeedback, ValidationError>;
}

impl UpdateFeedback {
    pub fn validate(self) -> Result<Self, ValidationError>;
}

pub async fn create_feedback(
    Json(input): Json<CreateFeedback>,
) -> Result<(StatusCode, Json<ValidatedFeedback>), (StatusCode, Json<ErrorResponse>)>;

pub fn app() -> Router;
```

## Правила

- Title после trim: 1–100 символов `.chars().count()`. Пустой → `EmptyTitle`, длинный → `TitleTooLong`.
- Body после trim: 1–2000 символов. Пустой → `EmptyBody`, длинный → `BodyTooLong`.
- Rating: 1–5, иначе `InvalidRating`.
- Create проверяет поля в перечисленном порядке и сохраняет trimmed title/body и исходные rating/category.
- Update без единого `Some` → `NoChanges`. Иначе проверяются только присутствующие поля в том же порядке; `None` остаётся `None`, `Some` строки нормализуются trim.
- `create_feedback`: успех → `(201 CREATED, Json(validated))`, ошибка → `(422 UNPROCESSABLE_ENTITY, Json(ErrorResponse { error: error.to_string() }))`.
- `app`: `POST /feedback` на create handler.

Serde отклоняет unknown fields из-за предоставленного атрибута; не убирайте его. Не добавляйте в DTO служебные поля сервера.

## Проверка

```bash
cargo test -p web-02-validation
cargo clippy -p web-02-validation --all-targets -- -D warnings
```
