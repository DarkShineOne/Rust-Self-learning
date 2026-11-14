# Домашнее задание: Tokio store и timeout

```rust
impl SharedStore {
    pub fn new() -> Self;
    pub async fn put(&self, feedback: Feedback) -> Result<(), StoreError>;
    pub async fn get(&self, id: u64) -> Option<Feedback>;
}

pub async fn receive_with_timeout(
    receiver: &mut mpsc::Receiver<Feedback>,
    timeout: Duration,
) -> Result<Feedback, ReceiveError>;

pub async fn collect_ratings(stores: Vec<SharedStore>) -> Vec<u8>;
```

## Контракт

- `put` добавляет запись или возвращает `DuplicateId`, не меняя старую.
- `get` возвращает клон записи либо `None`. Guard не должен удерживаться после возврата.
- `receive_with_timeout`: сообщение вовремя → `Ok(message)`, закрытый пустой канал → `Closed`, истечение duration → `Timeout`. Не создавайте дополнительные значения после timeout.
- `collect_ratings` обрабатывает независимые store конкурентно через Tokio tasks/JoinSet, собирает оценки всех записей и сортирует по возрастанию. Пустой список store даёт пустой Vec. Порядок завершения tasks не влияет на результат.

Не удерживайте lock через `tokio::spawn`, sleep или другой внешний await. Для краткой операции с map все необходимые await — это получение guard.

## Проверка

```bash
cargo test -p basics-14-async
cargo clippy -p basics-14-async --all-targets -- -D warnings
```
