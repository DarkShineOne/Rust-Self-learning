# Конкурентность и асинхронность

## Цели

Различить параллелизм ОС-потоков и конкурентное выполнение async tasks, безопасно разделять состояние через Tokio, использовать каналы и ограничивать ожидание по времени.

## Потоки, Send и Sync

`std::thread::spawn` создаёт ОС-поток. Замыкание обычно использует `move`, чтобы владеть данными и удовлетворять требованию `'static`.

`Send` означает возможность передать владение значением другому потоку. `Sync` означает безопасность разделяемых ссылок `&T` между потоками. Это автоматические трейты: компилятор выводит их по полям. `Rc` не Send, `RefCell` не Sync, поэтому они не подходят для общего состояния потоков.

`JoinHandle` позволяет дождаться потока и получить результат. Если его отсоединить, главная функция может завершиться раньше работы — в приложении нужен явный lifecycle.

Concurrency — организация нескольких продолжающихся операций, parallelism — одновременное исполнение. Async в первую очередь помогает при большом числе I/O-ожиданий, а не ускоряет CPU-bound вычисления сам по себе.

## Future и async/await

`async fn` возвращает тип, реализующий `Future`. Тело не выполняется, пока executor не начнёт опрашивать (poll) future. `.await` регистрирует продолжение и позволяет runtime переключиться на другую работу, если операция не готова.

```rust
async fn read_rating() -> u8 {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    5
}
```

`async` не означает автоматический новый поток. Обычная async fn выполняется в задаче вызывающего кода. `tokio::spawn` создаёт отдельную task, требуя обычно `Send + 'static` для future.

Runtime Tokio управляет задачами и драйверами I/O/timers. `#[tokio::main]` создаёт runtime для бинарника, `#[tokio::test]` — для async test. Не вызывайте блокирующий `std::thread::sleep` вместо асинхронного `tokio::time::sleep` в рабочем async handler.

## Tokio tasks и JoinSet

`JoinSet` собирает набор задач и позволяет получать результаты по мере завершения. Порядок результатов не обязан совпадать с порядком запуска, поэтому домашний контракт сортирует оценки.

Отсоединённая task усложняет обработку ошибок и shutdown. Храните handle или JoinSet там, где нужен контроль. Для CPU-bound работы используют `spawn_blocking` или отдельный пул, а не блокируют async worker.

## Каналы

`tokio::sync::mpsc` — multi-producer, single-consumer channel. Клонирование Sender увеличивает число производителей. Когда все Sender закрыты и очередь пуста, `recv()` возвращает `None`.

Канал передаёт владение сообщением, уменьшая разделяемую мутацию. Bounded channel ограничивает очередь и создаёт backpressure; unbounded channel может расти при медленном consumer.

`oneshot` подходит для одного ответа, `broadcast` — уведомлений многим подписчикам, `watch` — последнего состояния. Выбор зависит от семантики доставки.

## Tokio RwLock

`tokio::sync::RwLock` имеет async `read().await` и `write().await`. В отличие от std lock, guard можно удерживать через `.await`, но сама возможность не означает, что это хорошая идея.

В домашних `put`/`get` guard нужен только для операции с HashMap. Не вызывайте сеть или длительный timer, удерживая его. Получите данные, клонируйте необходимую небольшую запись, освободите guard, затем выполняйте I/O.

Для простого lock часто рекомендуют std Mutex, если нет await внутри critical section: короткая синхронная секция не требует async ожидания. В задании Tokio lock используется учебно и единообразно.

## Timeout и cancellation

`tokio::time::timeout(duration, future)` возвращает `Err(Elapsed)`, если future не завершилась вовремя. При этом future отбрасывается. Отмена происходит только в точках `.await` и не гарантирует отмену внешнего побочного эффекта, который уже произошёл.

В `receive_with_timeout` `None` канала означает `Closed`, `Elapsed` — `Timeout`, полученное значение — успех. Нужно не перепутать эти три исхода.

Для сетевого запроса дополнительно проектируют idempotency, retry и очистку ресурсов. Timeout — не универсальная транзакционная отмена.

## Типичные ошибки

- блокировать worker через std sleep или тяжёлый CPU цикл;
- передавать `Rc` в `tokio::spawn`;
- удерживать lock через внешний `.await`;
- игнорировать JoinError и cancellation;
- ожидать детерминированный порядок завершения tasks;
- считать `None` канала обычным timeout.

## Практика

[Домашнее задание](homework.md) покрывает async store, channel timeout и конкурентное чтение. Следующий урок объединяет знания модуля в консольный CRUD.

## Источники

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Tokio: Shared State](https://tokio.rs/tokio/tutorial/shared-state)
- [Tokio: Channels](https://tokio.rs/tokio/tutorial/channels)
- [The Rust Book: Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
