# CRUD в PostgreSQL

## Цели

Реализовать полноценный SQLx-репозиторий: create/read/update/delete, детерминированный список, фильтрацию, пагинацию и транзакцию. Отдельное внимание — корректные `NotFound` и отсутствие частично сохранённых данных.

## Маппинг строк

`#[derive(sqlx::FromRow)]` позволяет `query_as::<_, Feedback>` сопоставить колонки полям по именам. Это не проверяет SQL на этапе компиляции: ошибка имени или несовместимость типа проявится при выполнении запроса.

`query_as!` проверяет запрос через БД или offline-данные, но учебные заготовки используют runtime-вариант, чтобы компилироваться без DATABASE_URL. Production-команда может выбрать macros и `cargo sqlx prepare` после настройки инфраструктуры.

Всегда используйте `fetch_optional` там, где отсутствие строки является ожидаемым результатом. `fetch_one` вернёт `RowNotFound`, который можно преобразовать, но явный optional яснее отличает NotFound от ошибки соединения.

## Create и Returning

PostgreSQL `INSERT ... RETURNING` возвращает созданную строку вместе с identity id и defaults. Это избавляет от отдельного SELECT и гонки между вставкой и чтением.

`author_id` передаётся параметром. FK проверяет существование автора. `rating` и status проверяются constraints, но service по-прежнему обязан дать пользователю понятную ошибку до SQL.

Не формируйте SQL значениями через конкатенацию:

```rust,ignore
// Правильная форма: SQL содержит $1, значение передаётся через .bind(value).
```

Placeholders нумеруются в порядке bind. Ошибка порядка может компилироваться и давать неправильные результаты, поэтому интеграционные тесты обязательны.

## Get, Update и Delete

Get отсутствующего id возвращает `RepositoryError::NotFound`. Update и Delete используют `RETURNING`; если строки нет, результат тот же NotFound.

Update применяет только `Some`-поля. Можно построить динамический список assignments или аккуратно использовать `COALESCE($n, column)`, учитывая типы и семантику NULL. Наши колонки NOT NULL, а Option означает «не менять», поэтому COALESCE возможен, но updated_at должен обновляться при реальной операции update.

Пустой Update может вернуть текущую запись без UPDATE. Не превращайте его в установку всех полей NULL.

Delete возвращает удалённую запись. `ON DELETE RESTRICT` автора не относится к удалению самого feedback; attachments позже потребуют согласованной очистки.

## Динамические фильтры

`FeedbackFilter` содержит необязательные author_id/status. Динамический SQL можно собирать только из фиксированных фрагментов, например `" AND author_id = $2"`. Значения всегда bind, включая limit/offset.

`QueryBuilder` помогает управлять placeholders и bindings. Альтернатива — несколько явных запросов для небольшого числа сочетаний. Не подставляйте строку статуса напрямую.

ORDER BY id ASC обязателен: без него LIMIT/OFFSET не гарантирует стабильный порядок. В больших данных keyset pagination часто эффективнее большого OFFSET, но текущий контракт намеренно использует LIMIT/OFFSET для изучения основ.

`limit` разрешён от 1 до 100, `offset` от 0. Некорректные значения дают `InvalidPagination` до обращения к БД.

## Транзакции

`create_user_and_feedback` должен сохранить две записи атомарно:

1. начать transaction через `pool.begin().await?`;
2. создать user с ролью user и получить id;
3. создать feedback с этим author_id;
4. commit;
5. при любой ошибке не выполнять commit и дать transaction rollback при drop.

Если email нарушает unique constraint, пользователь уже существует в рамках попытки, feedback не должен остаться. Если второй INSERT нарушает rating, первый пользователь также должен откатиться.

Не держите transaction открытой во время внешнего HTTP/S3 вызова. Это удерживает соединение и locks. Интеграции требуют другого проектирования согласованности.

## Unique violations и ошибки

`sqlx::Error::Database` содержит код PostgreSQL и constraint. Нарушение уникальности обычно имеет SQLSTATE 23505. В следующих уроках его можно преобразовать в Conflict/409. Не сравнивайте человекочитаемый текст ошибки, если доступен код.

`RepositoryError::Database` сохраняет исходную sqlx::Error через `#[from]`. Это помогает логам и diagnostics, но публичный handler не должен возвращать её текст напрямую.

## Соединения и locks

`fetch_all` материализует Vec. Для большого результата можно использовать stream, но пагинация уже ограничивает размер. Возвращайте из repository данные после завершения query, не удерживая лишние resources.

Транзакция и запросы используют pool. Не создавайте новый pool в методе. PgPool clone разделяет соединения.

## Тестирование

Каждый `#[sqlx::test]` получает изолированную БД и применяет миграции. Helpers создают users через параметризованный SQL. Проверяются не только возвращаемые значения, но и дополнительные запросы к состоянию: rollback, пустой результат после delete, правильный порядок list.

Миграция снова оставлена нерешённой. Это намеренное повторение схемы перед реализацией CRUD.

## Типичные ошибки

- `fetch_one` без преобразования RowNotFound;
- фильтр через format!;
- list без ORDER BY;
- update создаёт новую запись;
- commit даже после ошибки;
- внешний API внутри DB transaction;
- передача sqlx::Error в HTTP response.

## Практика

[Домашнее задание](homework.md) задаёт точные сигнатуры. Следующий урок добавит безопасные password hashes и refresh tokens в эту модель.

## Источники

- [SQLx: query](https://docs.rs/sqlx/latest/sqlx/fn.query.html)
- [SQLx: QueryBuilder](https://docs.rs/sqlx/latest/sqlx/struct.QueryBuilder.html)
- [SQLx: Transactions](https://docs.rs/sqlx/latest/sqlx/struct.Transaction.html)
- [PostgreSQL: INSERT](https://www.postgresql.org/docs/current/sql-insert.html)
- [PostgreSQL: Transaction Isolation](https://www.postgresql.org/docs/current/transaction-iso.html)
