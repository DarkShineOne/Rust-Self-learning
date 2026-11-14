# Домашнее задание: публичная граница и тесты

Реализуйте методы в `src/domain.rs`, `src/storage.rs` и `src/lib.rs`.

```rust
impl Feedback {
    pub fn new(id: u64, title: &str, rating: u8) -> Result<Self, String>;
}

impl MemoryStore {
    pub fn new() -> Self;
    pub fn add(&mut self, feedback: Feedback) -> Result<(), String>;
    pub fn get(&self, id: u64) -> Option<&Feedback>;
    pub fn len(&self) -> usize;
}

pub fn create_feedback(
    store: &mut MemoryStore,
    id: u64,
    title: &str,
    rating: u8,
) -> Result<(), String>;
```

## Контракт

- `Feedback::new`: id больше 0, title после trim непустой, rating 1–5. Сохраняется trimmed title. Ошибки: `invalid id`, `title must not be empty`, `invalid rating`.
- `MemoryStore::add`: повторный id даёт `duplicate id`, существующие записи не меняются.
- `get` возвращает ссылку, `len` число записей, уже готовый `is_empty` использует `len`.
- `create_feedback` сначала валидирует запись, затем добавляет её. Ошибки домена и duplicate id возвращаются без изменения текста. При ошибке store не меняется.

Не делайте `records` публичным наружу: интеграционные тесты используют только методы. Unit test в `domain.rs` и doc test также должны компилироваться.

## Проверка

```bash
cargo test -p basics-12-modules
cargo test -p basics-12-modules --doc
cargo clippy -p basics-12-modules --all-targets -- -D warnings
```
