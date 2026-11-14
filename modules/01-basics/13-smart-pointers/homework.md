# Домашнее задание: Arc<RwLock<_>>

Реализуйте методы `SharedFeedbackStore`:

```rust
pub fn new() -> Self;
pub fn insert(&self, feedback: Feedback) -> Result<(), StoreError>;
pub fn get(&self, id: u64) -> Result<Feedback, StoreError>;
pub fn update_rating(&self, id: u64, rating: u8) -> Result<(), StoreError>;
pub fn len(&self) -> Result<usize, StoreError>;
pub fn is_empty(&self) -> Result<bool, StoreError>;
pub fn strong_count(&self) -> usize;
```

## Контракт

- Клоны store разделяют одно состояние, а новый `new()` создаёт независимое состояние.
- `insert`: существующий id → `DuplicateId`, исходная запись сохраняется.
- `get`: найденная запись клонируется, отсутствие → `NotFound`. Нельзя возвращать ссылку после drop guard.
- `update_rating`: сначала проверить диапазон 1–5, иначе `InvalidRating`; затем найти запись, иначе `NotFound`. При любой ошибке данные не меняются.
- `len`: число записей; `is_empty` возвращает `true` только при нулевом `len` и распространяет ошибку lock.
- Ошибка poisoning любого std lock преобразуется в `StoreError::Poisoned`, а не в panic.
- `strong_count` возвращает `Arc::strong_count(&self.inner)`; сам вызов не создаёт постоянную дополнительную ссылку.

Не используйте `unwrap()` на lock. Критические секции должны содержать только необходимую работу с map.

## Проверка

```bash
cargo test -p basics-13-smart-pointers
cargo clippy -p basics-13-smart-pointers --all-targets -- -D warnings
```
