# Rust: от основ до микросервиса обратной связи

Практический курс для тех, кто уже знаком с программированием, но начинает изучать Rust. Курс состоит из двух модулей и 29 уроков. Каждый урок содержит теорию в `readme.md`, задание в `homework.md`, компилируемую заготовку публичных методов и тесты на стандартном Rust test runner.

Готовых решений в репозитории нет. До выполнения задания тесты должны падать на `todo!()` — так они показывают, какой контракт ещё не реализован. Проверка выполненной работы включает тесты и код-ревью.

## Требования

- stable Rust с `rustup`, Cargo, Clippy и Rustfmt;
- для уроков 04–05 второго модуля и интеграционных тестов — PostgreSQL; удобный способ локального запуска описан в `compose.yaml` итогового урока;
- для S3 integration-тестов — самостоятельно установленный open-source S3-совместимый сервер, bucket и credentials. Подойдут MinIO, SeaweedFS или Ceph RGW; курс не устанавливает хранилище;
- Docker Desktop с Compose необязателен, если PostgreSQL уже установлен иначе. S3 в Compose намеренно не включён.

## Как проходить курс

1. Читайте уроки по порядку и запускайте небольшие примеры из теории.
2. Откройте `homework.md`: в нём перечислены сигнатуры, возвращаемые значения, ошибки и граничные случаи.
3. Найдите те же методы в `src`. Меняйте тела `todo!()`, сохраняя публичный контракт, если задание прямо не требует иного.
4. Запустите тесты пакета, затем Clippy и Rustfmt. Команда указана в каждом задании.
5. Перед отправкой на ревью проверьте не только успешные сценарии, но и ошибки, граничные значения и правила владения.

Проверка компиляции всех заготовок и тестов:

```bash
cargo test --workspace --no-run
cargo test --workspace -- --list
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
```

Команда `cargo test --workspace` на нерешённом курсе завершится ошибками из `todo!()`. Это ожидаемо и отличается от ошибки компиляции, которую необходимо исправить.

## Модуль 1. Базовый Rust

[Описание модуля](modules/01-basics/readme.md)

1. [Инструменты, Cargo, переменные и типы](modules/01-basics/01-tooling-and-types/readme.md)
2. [Функции и управление потоком](modules/01-basics/02-functions-and-control-flow/readme.md)
3. [Владение](modules/01-basics/03-ownership/readme.md)
4. [Заимствование и срезы](modules/01-basics/04-borrowing-and-slices/readme.md)
5. [Структуры и методы](modules/01-basics/05-structs-and-methods/readme.md)
6. [Перечисления, match и Option](modules/01-basics/06-enums-and-option/readme.md)
7. [Result и ошибки](modules/01-basics/07-result-and-errors/readme.md)
8. [Коллекции и строки](modules/01-basics/08-collections-and-strings/readme.md)
9. [Замыкания и итераторы](modules/01-basics/09-closures-and-iterators/readme.md)
10. [Обобщения и трейты](modules/01-basics/10-generics-and-traits/readme.md)
11. [Время жизни](modules/01-basics/11-lifetimes/readme.md)
12. [Модули и тестирование](modules/01-basics/12-modules-and-testing/readme.md)
13. [Умные указатели](modules/01-basics/13-smart-pointers/readme.md)
14. [Конкурентность и асинхронность](modules/01-basics/14-concurrency-and-async/readme.md)
15. [Консольный CRUD обратной связи](modules/01-basics/15-console-feedback-crud/readme.md)

## Модуль 2. Веб-разработка

[Описание модуля](modules/02-web/readme.md)

1. [HTTP и Axum](modules/02-web/01-http-and-axum/readme.md)
2. [JSON и валидация](modules/02-web/02-json-and-validation/readme.md)
3. [Архитектура приложения](modules/02-web/03-application-architecture/readme.md)
4. [PostgreSQL и SQLx](modules/02-web/04-postgresql-and-sqlx/readme.md)
5. [CRUD в PostgreSQL](modules/02-web/05-database-crud/readme.md)
6. [Аутентификация](modules/02-web/06-authentication/readme.md)
7. [Auth middleware](modules/02-web/07-auth-middleware/readme.md)
8. [Авторизация](modules/02-web/08-authorization/readme.md)
9. [Внешний HTTP API](modules/02-web/09-external-http-api/readme.md)
10. [S3 и вложения](modules/02-web/10-s3-attachments/readme.md)
11. [Тестирование веб-приложения](modules/02-web/11-web-testing/readme.md)
12. [Надёжность и безопасность API](modules/02-web/12-production-api/readme.md)
13. [OpenAPI и развёртывание](modules/02-web/13-openapi-and-deployment/readme.md)
14. [Итоговый микросервис](modules/02-web/14-final-feedback-service/readme.md)

## Итоговый результат

Студент самостоятельно реализует микросервис приёма обратной связи с PostgreSQL, S3, внешним HTTP API уведомлений, JWT access/refresh tokens и CRUD. Пользовательский API находится под `/api/public/v1`, модераторский — под `/api/private/v1`. Роутеры и OpenAPI-контракты разделены. Предполагается, что Nginx дополнительно ограничивает private API рабочей сетью/VPN или внутренним доменом, но приложение обязано самостоятельно проверять JWT и роли.

## Полезные источники

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Документация стандартной библиотеки](https://doc.rust-lang.org/std/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Axum](https://docs.rs/axum/)
- [SQLx](https://docs.rs/sqlx/)
- [AWS SDK for Rust](https://docs.rs/aws-sdk-s3/)
