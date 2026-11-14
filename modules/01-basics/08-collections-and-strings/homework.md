# Домашнее задание: индексы и Unicode

Реализуйте функции:

```rust
pub fn count_categories(records: &[FeedbackRecord]) -> HashMap<Category, usize>;
pub fn unique_words(text: &str) -> BTreeSet<String>;
pub fn index_by_id(records: Vec<FeedbackRecord>) -> HashMap<u64, FeedbackRecord>;
pub fn char_count(text: &str) -> usize;
```

## Контракт

- `count_categories` включает только встретившиеся категории; пустой вход даёт пустую map. Используйте `entry`.
- `unique_words` разделяет текст по любым символам, для которых `char::is_alphanumeric()` ложно. Пустые фрагменты отбрасываются. Слова переводятся в нижний регистр, дубликаты удаляются, порядок определяется `BTreeSet`.
- `index_by_id` потребляет Vec без клонирования записей. При одинаковых id последняя входная запись перезаписывает предыдущую.
- `char_count` возвращает `.chars().count()`, то есть число Unicode scalar values, а не байтов и не grapheme clusters.

## Проверка

```bash
cargo test -p basics-08-collections
cargo clippy -p basics-08-collections --all-targets -- -D warnings
```
