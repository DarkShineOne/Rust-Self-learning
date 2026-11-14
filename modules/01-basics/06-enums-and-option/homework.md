# Домашнее задание: состояния и Option

Реализуйте методы enum и функцию форматирования:

```rust
impl FeedbackStatus {
    pub fn can_transition_to(self, next: Self) -> bool;
    pub fn label(self) -> &'static str;
}

impl FeedbackCategory {
    pub fn from_code(code: &str) -> Option<Self>;
}

pub fn describe_rating(rating: Option<u8>) -> String;
```

## Матрица переходов

- Переход в текущий статус разрешён для всех статусов.
- `New` может перейти в `InReview` или `Rejected`.
- `InReview` может перейти в `Resolved` или `Rejected`.
- `Resolved` и `Rejected` терминальны: другой статус выбрать нельзя.
- Остальные переходы запрещены.

## Остальные контракты

- `label`: `new`, `in_review`, `resolved`, `rejected`.
- `from_code`: только точные строчные `bug`, `idea`, `question`, `praise`; пустая строка, другой регистр, пробелы и неизвестный код дают `None`.
- `describe_rating`: `Some(1..=5)` → `Rating: N/5`, `None` → `No rating`, остальные значения → `Invalid rating`.

Предпочитайте исчерпывающий `match`, чтобы добавление нового варианта вызывало ошибку компиляции до обновления логики.

## Проверка

```bash
cargo test -p basics-06-enums
cargo clippy -p basics-06-enums --all-targets -- -D warnings
```
