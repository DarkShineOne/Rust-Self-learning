# Домашнее задание: парсер команд с Result

Реализуйте четыре функции:

```rust
pub fn parse_rating(input: &str) -> Result<u8, CommandError>;
pub fn parse_positive_id(input: &str) -> Result<u64, CommandError>;
pub fn parse_command(input: &str) -> Result<FeedbackCommand, CommandError>;
pub fn parse_commands(inputs: &[&str]) -> Result<Vec<FeedbackCommand>, CommandError>;
```

Типы `CommandError` и `FeedbackCommand` уже определены в `src/lib.rs`.

## Правила разбора

Вход сначала обрезается через `trim()`. Пустая строка даёт `EmptyInput`.

- `parse_rating`: корректное десятичное `u8` от 1 до 5. Синтаксическая ошибка и выход за диапазон дают `InvalidRating(исходный_input)`.
- `parse_positive_id`: корректное `u64` больше 0. Ошибка даёт `InvalidId(исходный_input)`.
- `create <rating> <title>`: команда и rating разделяются whitespace. Отсутствие rating — `MissingRating`; отсутствие title после rating — `MissingTitle`. Title сохраняется trimmed и может содержать пробелы.
- `resolve <id>` и `reject <id>`: отсутствие id — `MissingId`. Дополнительные непустые аргументы после id — `UnexpectedArguments(эти_аргументы_trimmed)`.
- Неизвестное первое слово — `UnknownCommand(это_слово)`. Регистр команд значим.
- `parse_commands` обрабатывает элементы по порядку и возвращает первую ошибку. Пустой вход даёт пустой `Vec`.

Используйте `?` там, где одна операция зависит от результата другой. Не паникуйте на некорректном вводе.

## Проверка

```bash
cargo test -p basics-07-result
cargo clippy -p basics-07-result --all-targets -- -D warnings
```
