# Домашнее задание: цепочки итераторов

```rust
pub fn positive_titles(feedback: &[Feedback]) -> Vec<&str>;
pub fn average_by_status(feedback: &[Feedback], resolved: bool) -> Option<f64>;
pub fn top_ratings(feedback: &[Feedback], limit: usize) -> Vec<u8>;
pub fn count_matching(
    feedback: &[Feedback],
    predicate: impl FnMut(&Feedback) -> bool,
) -> usize;
```

## Контракт

- `positive_titles`: оценка 4 или 5, исходный порядок, возвращаются заимствованные `&str` без клонирования title.
- `average_by_status`: только записи с заданным `resolved`; среднее `f64`; если совпадений нет, `None`.
- `top_ratings`: копии оценок по убыванию, не более `limit`. `limit == 0` даёт пустой Vec, результат не содержит лишних элементов.
- `count_matching`: вызывает predicate для записей по порядку до конца и считает `true`. Поддерживается замыкание, изменяющее захваченное состояние.

Используйте итераторные адаптеры там, где они делают код понятнее. Не мутируйте исходный срез и не сортируйте его на месте.

## Проверка

```bash
cargo test -p basics-09-iterators
cargo clippy -p basics-09-iterators --all-targets -- -D warnings
```
