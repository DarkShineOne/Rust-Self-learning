# Авторизация и public/private API

## Цели

Научиться проверять роли и владение ресурсом, различать 401/403/404 и разделять пользовательские и модераторские роутеры. В учебных route tests `Extension<CurrentUser>` имитирует результат JWT middleware из урока 07; production не должен доверять роли из обычного request header.

## Матрица доступа

До кода сформулируйте правила:

- public `User` читает и изменяет только свои обращения;
- public update разрешён только в статусе New;
- private Moderator читает любые обращения и меняет статусы по workflow;
- Admin имеет тот же доступ к модерации в этом уроке; дополнительные admin-only операции появляются только по явному контракту;
- анонимный запрос отклонён ещё JWT middleware.

Матрица должна находиться в service/domain, а не только в отдельных `if` handler. Если новый endpoint забудет проверку, безопаснее централизованный guard.

## Роль и владение — разные проверки

`require_user` и `require_moderator` отвечают на вопрос о роли. `authorize_view` дополнительно сравнивает `actor.id` и `feedback.author_id`. Наличие валидного id в JWT не означает права на любой ресурс с таким же числом в URL.

IDOR проявляется, когда пользователь меняет `/feedback/1` на `/feedback/2` и получает чужие данные. Защита — проверка ownership для каждого объекта, а не непредсказуемые UUID и не скрытие кнопки в UI.

Private API тоже проверяет права. Отдельный домен и VPN не должны быть единственной защитой: сетевая ошибка или внутренний пользователь не превращаются в модератора.

## 403 и 404 для чужих ресурсов

В нашем public read чужая запись возвращает NotFound/404, чтобы не подтверждать её существование. Пользователь с ролью Moderator в public route получает Forbidden/403, потому что этот роутер предназначен для User.

Private route для обычного User возвращает 403 даже если resource принадлежит ему: проблема не владение, а недопустимая группа API. Отсутствие/недействительность JWT была бы 401 до этих функций.

Единая политика 404 против 403 должна быть осознанной. Иногда возврат 403 допустим, если существование ресурса публично; важно не зависеть от непоследовательных ответов для защиты данных.

## Public и private routers

Public и private handlers собираются отдельными функциями `public_app` и `private_app`. В итоговом проекте они монтируются под `/api/public/v1` и `/api/private/v1` и получают раздельные OpenAPI-контракты.

Разделение роутеров уменьшает риск случайно опубликовать moderation endpoint во внешнем списке. Но каждый private handler всё равно применяет `require_moderator`.

Вне задания предполагается Nginx: внешний домен проксирует public prefix, а private prefix доступен только из рабочей сети/VPN или через внутренний домен. Настройка Nginx не входит в домашнее задание и не тестируется.

## Workflow модерации

`authorize_moderation` сначала проверяет staff-роль, затем переход:

- same разрешён;
- New → InReview или Rejected;
- InReview → Resolved или Rejected;
- terminal status не меняется на другой.

Неверная роль → Forbidden, недопустимый переход → InvalidTransition. Handler позже превращает его в 409 Conflict. Нельзя разрешать клиенту public PATCH передавать произвольный status: модерация отделена от пользовательского редактирования текста.

## Extension в тестах

`Extension<CurrentUser>` здесь используется как test double upstream middleware. Route test добавляет конкретного actor к приложению и проверяет логику авторизации без повторения JWT из урока 07.

В production нельзя заменить это на `X-User-Id`/`X-Role` из интернета. Если identity приходит от доверенного proxy, приложение и proxy должны иметь защищённый канал и удалять любые поддельные входные headers. Наш итоговый сервис проверяет JWT сам.

## Defense in depth

Проверка в handler удобна для маршрутной группы, проверка в service обязательна для других вызовов, а условие `WHERE id = ? AND author_id = ?` в repository дополнительно ограничивает данные. Эти слои не исключают друг друга.

Однако SQL ownership condition не заменяет модерацию: moderator query намеренно не ограничен автором. Разные сценарии должны быть явными методами, а не одним универсальным «дай всё» с забытым флагом.

## Типичные ошибки

- проверять только наличие JWT;
- полагаться на скрытый UI или непредсказуемый ID;
- разрешать user выбирать role/status;
- считать private network авторизацией;
- возвращать чужой объект перед проверкой владельца;
- дублировать матрицу переходов в разных handlers.

## Практика

[Домашнее задание](homework.md) включает чистые authorization-функции и route tests обоих роутеров. Следующий урок добавит внешний HTTP API уведомлений модераторов.

## Источники

- [OWASP: Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html)
- [OWASP: Insecure Direct Object Reference Prevention](https://cheatsheetseries.owasp.org/cheatsheets/Insecure_Direct_Object_Reference_Prevention_Cheat_Sheet.html)
- [Axum: Extension](https://docs.rs/axum/latest/axum/struct.Extension.html)
- [RFC 9110: 401 and 403](https://www.rfc-editor.org/rfc/rfc9110)
