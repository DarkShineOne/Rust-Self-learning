# Наблюдаемость, надёжность и безопасность API

## Цели

Научиться возвращать RFC 9457 Problem Details, распространять request ID, ограничивать тело запроса и настраивать middleware без утечки секретов. Рассмотреть readiness/liveness, graceful shutdown и базовый CORS.

## Problem Details

`application/problem+json` стандартизирует машинное представление ошибки. Основные поля:

- `type` — URI описания класса проблемы;
- `title` — краткое стабильное название;
- `status` — HTTP status;
- `detail` — конкретное безопасное описание;
- `instance` — URI/путь конкретного случая;
- `request_id` — расширение для сопоставления с логами.

Внутренний SQL, stack trace, DATABASE_URL, Authorization и S3 credentials не входят в detail. Публичный 500 содержит общий `internal server error`, а подробная причина записывается в защищённый structured log.

Не возвращайте одинаковый detail для 400/401/403/404: клиенту нужен корректный класс ошибки. Но для чужого ресурса политика 404 может намеренно скрывать его существование.

## Request ID

`SetRequestIdLayer` сохраняет подходящий входной `x-request-id` или создаёт новый через `MakeRequestUuid`. `PropagateRequestIdLayer` копирует ID в response. Одинаковое значение в Problem Details и логах связывает запрос с диагностикой.

Middleware имеет порядок. Layer, добавленный последним, оборачивает предыдущие и видит запрос раньше. Проверьте тестом, что ID не только присутствует в extension/header handler, но и возвращается клиенту.

Не используйте request ID как секрет или уникальный ключ БД. Это correlation identifier, которому не следует слепо доверять в auth-решениях.

## Structured tracing

`TraceLayer::new_for_http` создаёт spans для HTTP. `tracing` позволяет добавить method, path, status и request ID. Structured fields удобнее свободного текста для поиска.

Не записывайте тело запроса по умолчанию: там могут быть пароли и персональные данные. Header allowlist не должен включать Authorization/Cookie. Presigned URL и refresh token также относятся к секретам.

Логирование должно иметь уровень: request failures 4xx не обязаны быть error, внутренние сбои — error/warn с контекстом. Избыточные debug logs в production могут раскрывать данные и снижать производительность.

## Body и multipart limits

`DefaultBodyLimit::max(16)` в учебном Router ограничивает body для extractor. Больший запрос получает 413 до обработки handler. В реальном API предел выбирается по ресурсу: JSON feedback и attachment имеют разные limits.

Multipart требует ограничения каждого файла, общего body, количества частей и времени чтения. Проверка после чтения всего файла не защищает память.

413 относится к клиентской ошибке размера, timeout внешнего сервиса обычно 504, недоступность обязательной зависимости — 503. Эти варианты представлены в ErrorKind.

## CORS

CORS управляет браузерным доступом cross-origin и не является авторизацией. Server-to-server запросы и curl не ограничиваются CORS.

В задании разрешён только `https://feedback.example.com`, методы GET/POST и headers Content-Type/Authorization. `allow_origin` получает конкретный HeaderValue. Не используйте wildcard вместе с credentials без понимания ограничений браузера и рисков.

Nginx и private network остаются отдельной защитой private API; CORS не заменяет её.

## Liveness и readiness

Liveness проверяет, что процесс не требует перезапуска. Readiness определяет готовность принимать трафик: миграции применены, PostgreSQL доступен, обязательная конфигурация загружена. Не делайте readiness зависимым от необязательного notification API, если сервис может принимать обращения без него.

Health response не раскрывает connection strings и внутренние ошибки. Детали идут в логи с request ID.

## Graceful shutdown

`axum::serve(...).with_graceful_shutdown(...)` прекращает приём новых соединений после сигнала и позволяет активным запросам завершиться. Нужны timeout, завершение фоновых задач и закрытие ресурсов по lifecycle.

На SIGTERM orchestrator ожидает ограниченное время. Долгие внешние запросы должны иметь собственные deadlines. S3/HTTP retries не должны продолжаться бесконечно после shutdown.

## Rate limiting и TLS

Public login/register нуждаются в rate limiting на уровне приложения или gateway. CORS и body limit не заменяют защиту от brute force. TLS обязателен при передаче паролей и токенов; локальный HTTP используется только для разработки.

Секреты хранятся в secret manager/переменных среды, не в Docker image и не в репозитории. Backup и логи защищаются отдельно.

## Типичные ошибки

- внутренний sqlx::Error в публичном detail;
- request ID только в логах, но не в ответе;
- permissive CORS как мера безопасности;
- лимит после буферизации файла;
- readiness всегда 200 без зависимостей;
- отсутствие timeout и shutdown;
- Authorization в access logs.

## Практика

[Домашнее задание](homework.md) требует Problem Details и middleware. Следующий урок задокументирует раздельные public/private API и подготовит контейнерный запуск.

## Источники

- [RFC 9457: Problem Details](https://www.rfc-editor.org/rfc/rfc9457)
- [Tower HTTP: Request ID](https://docs.rs/tower-http/latest/tower_http/request_id/index.html)
- [Tower HTTP: Trace](https://docs.rs/tower-http/latest/tower_http/trace/index.html)
- [Axum: DefaultBodyLimit](https://docs.rs/axum/latest/axum/extract/struct.DefaultBodyLimit.html)
- [MDN: CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/CORS)
