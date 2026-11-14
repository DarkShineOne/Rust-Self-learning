# OpenAPI и запуск в эксплуатации

## Цели

Научиться генерировать OpenAPI через Utoipa, разделять public/private документы, проверять конфигурацию без утечки секретов и собирать минимальный контейнер с непривилегированным пользователем.

## OpenAPI как контракт

Derive-макрос `OpenApi` из Utoipa собирает документ из зарегистрированных path-аннотаций и типов с `ToSchema`. Описание должно соответствовать фактическим routes, request/response DTO и статусам. Если handler изменился, аннотация и тесты должны измениться вместе с ним.

PublicApiDoc содержит только `/api/public/v1/...`, PrivateApiDoc — только `/api/private/v1/...`. Внутренние модераторские схемы не публикуются во внешнем документе. Это не защита от доступа, а управление контрактом и уменьшение раскрытия информации.

В итоговом проекте добавьте bearer security scheme, 401/403 responses и корректные requestBody для multipart. Документация не заменяет middleware и авторизацию.

## Документы и код

`PublicApiDoc::openapi()` возвращает тип `utoipa::openapi::OpenApi`. Его можно сериализовать в JSON и отдать через Json. Route tests разбирают документ и проверяют paths, что надёжнее визуального сравнения строки.

`#[utoipa::path]` содержит method, path, request_body и responses. Параметр `{id}` описывается через `params`. Не нужно дублировать всю логику в комментарии; фиксируйте внешний контракт.

Версия учебного API — 0.1.0. При публичной эволюции API используйте осмысленное версионирование и backward compatibility.

## Проверка конфигурации

Приложение должно падать при старте с понятной ошибкой, если отсутствует обязательное значение. Не ждите первого пользовательского запроса, чтобы обнаружить пустой bucket или слабый JWT secret.

`AppConfig::validate` проверяет:

- PostgreSQL URL начинается с `postgres://` или `postgresql://`;
- S3 endpoint и notification URL — абсолютные HTTP(S) URL;
- bucket и credentials непустые;
- JWT secret не короче 32 байт.

Полный парсер URL предпочтительнее проверки префикса в production. Для учебного задания достаточно схемы и непустого host по контракту тестов.

Ручной Debug скрывает DATABASE_URL и все credentials. Не используйте `#[derive(Debug)]` для секретной конфигурации и не печатайте структуру при старте.

## Multi-stage Dockerfile

Первый stage компилирует release binary, второй содержит только runtime и скомпилированный файл. Исходники, Cargo cache и компилятор не нужны в конечном image.

`USER 10001` запускает процесс без root. Это уменьшает последствия компрометации. Не встраивайте `.env` и секреты через `COPY` или `ARG`: они могут остаться в слоях/image history.

В реальном проекте добавьте `.dockerignore` с `target`, `.git`, `.env`, закрепление базовых images по digest, обновление OS packages, минимальные CA certificates при HTTPS и healthcheck по требованиям платформы.

Dockerfile курса собирает один пакет workspace. Для итогового проекта имя binary будет другим.

## Compose и зависимости

Compose содержит только PostgreSQL с healthcheck и volume. S3 остаётся внешним сервисом, установленным разработчиком. Notification API тоже является внешней зависимостью и не подменяется скрытым контейнером.

`depends_on` с condition service_healthy помогает задать порядок запуска, но не заменяет reconnect/readiness в приложении. Контейнер БД может перезапуститься позже.

Volume сохраняет данные. Не добавляйте автоматический `down -v` в инструкции: он удалит базу. Для integration tests используются отдельные временные БД, а не основной volume разработки.

## Миграции при запуске

Миграции должны применяться явно и наблюдаемо: отдельным job/init container или контролируемым startup шагом. Несколько реплик не должны бесконтрольно менять схему одновременно. SQLx использует журнал миграций и locks в пределах своей реализации, но deployment policy остаётся задачей команды.

Readiness не сообщает 200, пока обязательные миграции и pool не готовы. Liveness не должен перезапускать процесс только из-за временно недоступной БД.

## Nginx и домены

Вне задания считается, что Nginx проксирует только public prefix на внешнем домене. Private prefix доступен через рабочую сеть/VPN или отдельный внутренний домен. DNS/VPN/Nginx настраивает инфраструктурный разработчик.

Даже если private OpenAPI физически доступен только внутри, приложение обязано проверять JWT/роли. Сеть — дополнительный периметр, не identity provider.

## Чек-лист перед эксплуатацией

- миграции и rollback/backup план проверены;
- секреты поступают из secret manager и не логируются;
- TLS, rate limiting, CORS и body limits настроены;
- PostgreSQL/S3 credentials имеют минимальные права;
- graceful shutdown и timeouts проверены;
- readiness/liveness различаются;
- public/private routers и OpenAPI разделены;
- логи содержат request ID, но не секреты;
- антивирусная проверка и orphaned S3 cleanup определены по рискам.

## Практика

[Домашнее задание](homework.md) завершает подготовку к итоговому проекту. Следующий урок содержит только спецификацию, заготовки и acceptance-тесты полного микросервиса.

## Источники

- [Utoipa](https://docs.rs/utoipa/)
- [OpenAPI Specification](https://spec.openapis.org/oas/latest.html)
- [Docker: Multi-stage builds](https://docs.docker.com/build/building/multi-stage/)
- [Docker Compose: Startup order](https://docs.docker.com/compose/how-tos/startup-order/)
- [SQLx: Migrations](https://docs.rs/sqlx/latest/sqlx/macro.migrate.html)
