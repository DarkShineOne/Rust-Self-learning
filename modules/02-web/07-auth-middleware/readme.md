# Middleware и защищённые маршруты

## Цели

Научиться проверять JWT централизованно, извлекать Bearer token и превращать claims в `CurrentUser` через Axum extractor. Авторизация по ролям остаётся следующим уроком: аутентифицированный пользователь ещё не получает все права.

## Bearer в Authorization

Схема выглядит как `Authorization: Bearer <token>`. Название схемы case-insensitive, token непустой и не содержит дополнительных слов. Отсутствие заголовка и malformed header имеют разные внутренние варианты AuthError, но одинаковый HTTP 401.

Не принимайте token из query string: URL попадает в access logs, историю и proxy. Не логируйте Authorization header.

HeaderMap содержит typed HTTP headers. Преобразование в `&str` может завершиться ошибкой из-за не-ASCII; это `InvalidAuthorization`.

## JWT validation

Проверка должна включать:

- подпись через ключ и только HS256;
- точный issuer;
- ожидаемый audience;
- expiration;
- issued-at не из будущего по контракту урока;
- валидный subject при переходе к CurrentUser.

`jsonwebtoken::Validation::new(Algorithm::HS256)` фиксирует алгоритм. Методы `set_issuer`/`set_audience` добавляют проверки. Никогда не принимайте `none` и не выбирайте ключ/алгоритм только потому, что так написано во входном header.

Для детерминированного задания автоматическая exp-проверка библиотеки отключается, а claims.exp сравнивается с параметром `now`. `now >= exp` означает ExpiredToken. `iat > now` означает InvalidToken. Допуск clock skew в production задаётся явно, а не скрывается.

Ошибка подписи/формата/issuer/audience — InvalidToken. Не сообщайте внешнему клиенту детали, помогающие перебирать токены.

## Claims и CurrentUser

`sub` должен быть десятичным положительным `u64`; 0 и нечисловое значение — InvalidSubject. Role десериализуется только известным вариантом Serde. Unknown role делает token недействительным ещё на decode.

`CurrentUser` содержит только нужные приложению id/role. Не передавайте всю строку token в domain services.

Claims доверенны только после успешной проверки подписи и остальных правил. Сам по себе base64 payload не является доказательством.

## FromRequestParts

Axum extractor позволяет повторить проверку для каждого защищённого handler:

```rust,ignore
async fn profile(user: CurrentUser) -> Json<CurrentUserResponse> {
    // user уже проверен middleware/extractor
    todo!()
}
```

`FromRequestParts<AuthState>` не потребляет тело и получает доступ к headers и состоянию. Rejection возвращает 401 с JSON ErrorResponse. Runtime clock берётся из `Utc::now()`, а чистая `validate_token` остаётся тестируемой с заданным now.

В большой системе проверку можно оформить как `middleware::from_fn_with_state`, но extractor хорошо выражает типизированную зависимость handler. Не дублируйте проверку в каждом handler вручную.

## 401 и 403

401 Unauthorized по HTTP-семантике означает отсутствие или недействительность аутентификации. 403 Forbidden означает, что личность известна, но действие запрещено. Отсутствие token → 401, валидный user в private moderator API → 403.

Слово Unauthorized в названии кода не делает его ответом об отсутствии авторизации в смысле прав. Это важно для единого API-контракта.

## Router и state

`AuthState` Clone, config находится в Arc. `protected_app` передаёт состояние в Router. Нельзя хранить текущего пользователя в AuthState: state общий для запросов, CurrentUser создаётся на каждый request.

Для route tests JWT создаётся только тестовым helper с известным секретом. Это не часть production login. Валидный token проверяет весь путь handler/extractor, а не только функцию decode.

## Refresh и access

Middleware проверяет только access JWT. Refresh token не должен приниматься как access: в итоговом проекте добавьте явный token type claim или используйте разные механизмы/ключи и проверяйте их. Opaque refresh из урока 06 уже не пройдёт JWT decode.

Logout refresh не отзывает stateless access. Его TTL должен быть коротким, а при компрометации ключей нужна ротация ключей и политика недействительности старых подписей.

## Типичные ошибки

- доверять payload без подписи;
- разрешать любой алгоритм;
- не проверять issuer/audience;
- принимать Bearer с пустым token;
- класть CurrentUser в общий state;
- возвращать 403 при недействительном token;
- логировать Authorization.

## Практика

[Домашнее задание](homework.md) задаёт чистые функции проверки и Axum extractor. В следующем уроке CurrentUser участвует в public/private authorization.

## Источники

- [jsonwebtoken: Validation](https://docs.rs/jsonwebtoken/latest/jsonwebtoken/struct.Validation.html)
- [Axum: FromRequestParts](https://docs.rs/axum/latest/axum/extract/trait.FromRequestParts.html)
- [RFC 6750: Bearer Token Usage](https://www.rfc-editor.org/rfc/rfc6750)
- [RFC 8725: JWT BCP](https://www.rfc-editor.org/rfc/rfc8725)
