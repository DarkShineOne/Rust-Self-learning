# Тестирование веб-приложения

## Цели

Собрать тестовую стратегию по слоям: чистые доменные tests, service tests с mocks, route tests через Tower, изолированные PostgreSQL tests и HTTP/S3 integration tests. Важно проверять побочные эффекты и ошибки, а не только успешный response.

## Пирамида и призма тестов

Внизу — быстрые unit tests чистых функций. Выше — service tests с небольшими hand-written mocks, route tests без TCP, затем integration tests реальных адаптеров и немного end-to-end сценариев. Это не жёсткое численное правило: важно, чтобы ошибка обнаруживалась на самом нижнем подходящем уровне.

Не нужно мокировать всё. Для SQL-запроса mock repository не найдёт ошибку placeholder или constraint; нужна PostgreSQL. Для JSON handler mock server не найдёт ошибку маршрута; нужен oneshot.

## Hand-written mocks

`FeedbackRepository` и `Notifier` реализуются тестовыми структурами с `Arc<Mutex<State>>`. Mock сохраняет полученные значения и может вернуть запланированную ошибку. Этого обычно достаточно, поэтому дополнительный mock framework не обязателен.

Mock должен проверять контракт, но не имитировать всю production-реализацию. Иначе тест начинает зависеть от сложной внутренней логики и дублирует её.

В задании нужно проверить:

- validation не вызывает repository/notifier;
- repository error не вызывает notifier;
- успешная запись приводит к одному уведомлению;
- ошибка уведомления не откатывает запись и отражается в `CreationOutcome`.

## Partial success

Внешний HTTP сбой после INSERT нельзя откатить силами Reqwest. Контракт явно возвращает созданную запись и `notification_error`. Это учебный пример явного частичного успеха, а не универсальный HTTP response.

В production такой outcome можно превратить в 202 Accepted с последующей обработкой outbox или вернуть другой документированный статус. Скрытая строка ошибки не должна содержать секреты. Для ревью важно, что поведение согласовано и протестировано.

## Route tests

`tower::ServiceExt::oneshot` вызывает Router напрямую. AppState создаётся с mock-зависимостями, поэтому тест проверяет extraction, mapping service error, статус 201/422 и JSON без сети.

Для GET/POST используйте корректный Content-Type и тело. Проверяйте не только status, но и содержимое ответа и вызовы mocks после запроса.

`Router::clone` позволяет выполнить несколько сценариев. Для независимости каждый test создаёт новые mocks и state.

## PostgreSQL tests

`#[sqlx::test]` создаёт изолированную базу и применяет миграции. Такие tests требуют настоящий DATABASE_URL и права создания БД. Не заменяйте их SQLite: SQL диалект, identity, constraints и транзакции отличаются.

Проверяйте состояние дополнительными запросами. Например, после ошибки второго INSERT транзакция должна отсутствовать не только в возвращаемой ошибке, но и в count строк.

## HTTP и S3 doubles

Wiremock проверяет реальные HTTP-запросы Reqwest к локальному серверу: headers, JSON, retry, delay. Это сильнее, чем простая подмена метода `notify`, потому что сериализация и HTTP client участвуют в тесте.

Для S3 unit tests обычно достаточно `ObjectStorage` mock. Отдельный ignored test настоящего bucket проверяет SDK configuration и подпись. Не используйте локальную папку как S3 integration test: она не проверяет S3 API.

## Acceptance tests

Итоговый сервис должен иметь сценарии через публичные handlers/HTTP API:

- register → login → create/read/update/delete;
- user не получает чужую запись;
- user получает 403 на private router даже без Nginx;
- moderator/admin выполняют модерацию;
- refresh rotation/logout;
- upload/download URL/delete attachment;
- сбой HTTP/S3 не нарушает зафиксированную согласованность.

Acceptance-тесты используют публичный контракт и не зависят от приватных полей. Их можно перенести в самостоятельную реализацию студента.

## Надёжность тестов

Избегайте sleep для синхронизации: передавайте время, используйте channels и детерминированные barriers. Timeout-тестам нужны разумные пределы. Не меняйте глобальные env в параллельных tests. Создавайте уникальные файлы/keys/DB и очищайте ресурсы.

`cargo test -- --list` показывает обнаруженные тесты без их запуска. `cargo test --no-run` проверяет компиляцию. Игнорируемые integration tests перечисляются отдельно и запускаются только при готовых зависимостях.

## Типичные ошибки

- mock повторяет production алгоритм;
- assertions только на Ok/Err без варианта;
- общий mutable state между тестами;
- route tests через реальный порт без необходимости;
- SQL mock вместо PostgreSQL integration;
- скрытый `#[ignore]` без документированной команды;
- тестирование приватных полей вместо публичного поведения.

## Практика

[Домашнее задание](homework.md) требует реализации service/handler, а mocks уже подготовлены. В следующем уроке ошибки будут переведены в Problem Details и снабжены request ID.

## Источники

- [Rust Book: Test Organization](https://doc.rust-lang.org/book/ch11-03-test-organization.html)
- [SQLx: test macro](https://docs.rs/sqlx/latest/sqlx/attr.test.html)
- [Tower ServiceExt](https://docs.rs/tower/latest/tower/trait.ServiceExt.html)
- [Wiremock](https://docs.rs/wiremock/)
