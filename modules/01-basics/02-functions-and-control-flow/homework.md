# Домашнее задание: выражения и циклы

Реализуйте четыре публичные функции в `src/lib.rs`, сохранив сигнатуры.

```rust
pub fn classify_number(number: i32) -> &'static str;
pub fn fizz_buzz(number: u32) -> String;
pub fn sum_even_below(limit: u32) -> u32;
pub fn collatz_steps(number: u64) -> u32;
```

## Требования

- `classify_number`: строго `"negative"`, `"zero"` или `"positive"`.
- `fizz_buzz`: кратно 15 — `FizzBuzz`, кратно 3 — `Fizz`, кратно 5 — `Buzz`, иначе число в десятичной записи. Для 0 результат `FizzBuzz`.
- `sum_even_below`: сумма чётных `0, 2, ...`, строго меньших `limit`. Для `limit < 2` результат 0. Тестовые значения не вызывают переполнения.
- `collatz_steps`: для чётного `n` следующее значение `n / 2`, для нечётного `3n + 1`. Вернуть количество шагов до `n == 1`; для 1 ответ 0. Вход не меньше 1, тестовые последовательности не переполняют `u64`.

Используйте возможность `if` возвращать значение хотя бы в одной функции. Не используйте рекурсию в `collatz_steps`.

## Проверка

```bash
cargo test -p basics-02-control-flow
cargo clippy -p basics-02-control-flow --all-targets -- -D warnings
```
