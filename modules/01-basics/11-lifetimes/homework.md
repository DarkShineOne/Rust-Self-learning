# Домашнее задание: заимствованные представления

```rust
pub fn longest<'a>(left: &'a str, right: &'a str) -> &'a str;

impl<'a> TextView<'a> {
    pub fn new(text: &'a str) -> Self;
    pub fn first_line(&self) -> &'a str;
    pub fn into_inner(self) -> &'a str;
}

pub fn lines_containing<'a>(text: &'a str, needle: &str) -> Vec<&'a str>;
```

## Контракт

- `longest` сравнивает `.len()` в байтах. При равенстве возвращает `left`. Результат должен указывать на один из входов без новой аллокации.
- `TextView::new` сохраняет исходную ссылку.
- `first_line` возвращает первую строку по правилам `str::lines()`, без `\n` и завершающего `\r`. Для пустого текста вернуть пустой срез. Результат `'a` должен оставаться доступным после drop самого `TextView`.
- `into_inner` возвращает исходный текст.
- `lines_containing` использует регистрозависимый `contains`. Пустой `needle` возвращает все `text.lines()`. Порядок исходный, ссылки указывают в `text`.

Не меняйте `'a` на `'static` и не заменяйте ссылки на `String`.

## Проверка

```bash
cargo test -p basics-11-lifetimes
cargo clippy -p basics-11-lifetimes --all-targets -- -D warnings
```
