# Домашнее задание: Repository и generics

Реализуйте `MemoryRepository::new`, `MemoryRepository::values`, все методы `Repository` для `MemoryRepository` и generic-функцию:

```rust
pub fn find_max_by_key<T, K, F>(items: &[T], key: F) -> Option<&T>
where
    K: Ord,
    F: FnMut(&T) -> K;
```

## Контракт хранилища

- `new` и `default` создают пустой репозиторий.
- `insert`: новый id добавляется; существующий id даёт `DuplicateId`, старая запись не меняется.
- `get`: ссылка на запись или `None`.
- `update`: существующая запись заменяется целиком; отсутствующий id даёт `NotFound`, ничего не вставляется.
- `delete`: удаляет и возвращает owned запись либо `None`.
- `len`/`is_empty`: число записей и признак пустоты.
- `values`: ссылки на записи по возрастанию id, без клонирования записей.

`Default` уже реализован через derive и использует тот же инвариант пустой map.

## Generic-поиск

- Пустой вход даёт `None`.
- Возвращается ссылка на элемент с максимальным ключом.
- При равенстве максимальных ключей выбирается первый такой элемент.
- Не клонируйте `T`. Замыкание можно вызывать повторно; его побочные эффекты не входят в контракт порядка вызовов.

## Проверка

```bash
cargo test -p basics-10-traits
cargo clippy -p basics-10-traits --all-targets -- -D warnings
```
