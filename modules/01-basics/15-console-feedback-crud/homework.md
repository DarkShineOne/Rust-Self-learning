# Домашнее задание: консольный CRUD

Реализуйте все `todo!()` в `src/lib.rs`: `FeedbackStatus::can_transition_to`, конструктор repository, `load`/`save`, методы `FeedbackService` и `run_cli`. `src/main.rs` уже является тонкой оболочкой.

## Repository

```rust
pub fn JsonFileRepository::new(path: PathBuf) -> Self;
fn load(&self) -> Result<Vec<Feedback>, ServiceError>;
fn save(&self, feedback: &[Feedback]) -> Result<(), ServiceError>;
```

- `new` сохраняет путь, `path()` уже реализован.
- `load`: отсутствующий файл → `Ok(Vec::new())`; остальные I/O и JSON ошибки → `Storage(message)`.
- `save`: сериализует весь срез в pretty JSON и записывает в файл. Ошибки → `Storage`.
- Не создавайте родительские каталоги и не трактуйте malformed JSON как пустое хранилище.

## Service

```rust
pub fn FeedbackService::new(repository: R) -> Self;
pub fn create(&self, draft: NewFeedback) -> Result<Feedback, ServiceError>;
pub fn list(&self) -> Result<Vec<Feedback>, ServiceError>;
pub fn get(&self, id: u64) -> Result<Feedback, ServiceError>;
pub fn update(&self, id: u64, update: UpdateFeedback) -> Result<Feedback, ServiceError>;
pub fn set_status(&self, id: u64, status: FeedbackStatus) -> Result<Feedback, ServiceError>;
pub fn delete(&self, id: u64) -> Result<Feedback, ServiceError>;
```

- Title и body проверяются после `trim()` на непустоту, сохраняются trimmed. Пустые дают `InvalidTitle`/`InvalidBody`, рейтинг вне 1–5 — `InvalidRating`.
- `create`: id равен максимальному существующему + 1, пустое хранилище начинается с 1; статус `New`.
- `list`: сортировка по id. `get`/`update`/`delete` отсутствующего id → `NotFound`.
- `update`: меняются только `Some`-поля, `None` сохраняет прежнее. Все переданные значения валидируются до сохранения.
- `set_status`: матрица из `can_transition_to`; недопустимый переход → `InvalidTransition` без изменения файла.
- `delete`: удаляет одну запись, сохраняет оставшиеся и возвращает удалённую.
- Каждая успешная мутация сохраняется через repository. Возвращаются owned записи.

## CLI

```rust
pub fn run_cli(args: &[String], repository: JsonFileRepository)
    -> Result<String, ServiceError>;
```

`args` не содержит имени программы. Поддерживаются только:

```text
create <title> <body> <rating>
list
get <id>
update <id> <title> <body> <rating>
status <id> <new|in_review|resolved|rejected>
delete <id>
```

- Title/body передаются одним аргументом каждый; в shell строки с пробелами берутся в кавычки.
- `update` через CLI заполняет все три `Some`-поля.
- Успешный результат сериализуется в компактный JSON: созданная/изменённая/удалённая запись или массив list.
- Неверное число аргументов, неизвестная команда, некорректные id/rating/status → `InvalidCommand(args.join(" "))`.
- После корректного разбора доменные ошибки service возвращаются без превращения в `InvalidCommand`.

## Проверка

```bash
cargo test -p basics-15-console
cargo clippy -p basics-15-console --all-targets -- -D warnings
```
