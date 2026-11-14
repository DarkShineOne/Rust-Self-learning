# Домашнее задание: модель Feedback

Дана структура:

```rust
pub struct Feedback {
    pub id: u64,
    pub title: String,
    pub rating: u8,
}
```

Реализуйте её публичные методы:

```rust
pub fn new(id: u64, title: String, rating: u8) -> Result<Self, String>;
pub fn is_positive(&self) -> bool;
pub fn update_rating(&mut self, rating: u8) -> Result<(), String>;
pub fn into_title(self) -> String;
```

## Инварианты и ошибки

- `id` должен быть больше 0, иначе `Err("id must be greater than zero".into())`.
- `title` после `trim()` не должен быть пустым, иначе `Err("title must not be empty".into())`.
- `rating` должен быть 1–5, иначе `Err("rating must be between 1 and 5".into())`.
- Проверки конструктора выполняются в порядке `id`, `title`, `rating`.
- При успешном создании заголовок сохраняется trimmed, остальные поля совпадают со входом.
- `is_positive` истинен только для 4 и 5.
- `update_rating` использует то же сообщение об ошибке оценки. При ошибке старое значение остаётся неизменным, при успехе возвращается `Ok(())`.
- `into_title` возвращает owned title без клонирования.

## Проверка

```bash
cargo test -p basics-05-structs
cargo clippy -p basics-05-structs --all-targets -- -D warnings
```
