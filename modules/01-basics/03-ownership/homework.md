# Домашнее задание: move и Clone

Реализуйте методы, не меняя owned-параметры на ссылки: цель урока — работа с владением.

```rust
pub fn normalize_title(title: String) -> String;
pub fn take_and_return(text: String) -> (String, usize);
pub fn duplicate_title(title: String) -> (String, String);
```

## Контракт

- `normalize_title` удаляет Unicode whitespace по краям и преобразует остальной текст в верхний регистр. Внутренние пробелы не меняются.
- `take_and_return` возвращает кортеж `(исходная_строка, длина_в_байтах)`. Длина вычисляется до передачи владения вызывающему коду, содержимое не меняется.
- `duplicate_title` возвращает два одинаковых независимых `String`. Изменение одного результата не должно менять другой. Используйте явный Clone там, где нужны два владельца.

Не используйте `unsafe`. Не меняйте публичные сигнатуры ради сохранения исходной переменной: тесты также проверяют идиоматичное использование move на этапе компиляции.

## Проверка

```bash
cargo test -p basics-03-ownership
cargo clippy -p basics-03-ownership --all-targets -- -D warnings
```
