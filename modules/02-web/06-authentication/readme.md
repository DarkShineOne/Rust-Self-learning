# Регистрация и аутентификация

## Цели

Различить аутентификацию и авторизацию, безопасно хешировать пароли, выпускать короткоживущий JWT access token и rotating opaque refresh token. Учебный AuthService хранит состояние в памяти, чтобы сосредоточиться на контрактах; итоговый проект переносит users/refresh tokens в PostgreSQL.

## Authentication и Authorization

Аутентификация отвечает «кто вы?» и проверяет пароль/token. Авторизация отвечает «что вам разрешено?» и проверяет роль/владельца. Успешный login сам по себе не дает права читать чужие обращения.

Public registration создаёт только `Role::User`. Поле role нельзя принимать из JSON. Модераторы назначаются отдельным доверенным процессом вне этой задачи.

## Почему Argon2id

Пароль нельзя хранить в открытом виде или как одиночный SHA-256. Быстрый hash позволяет перебирать словарь. Argon2id — memory-hard password hashing algorithm с солью и параметрами стоимости.

Rust crate `argon2` по умолчанию предоставляет Argon2id. `SaltString::generate` использует криптографически случайный источник. Результат PHC-строки содержит алгоритм, параметры, соль и hash, поэтому для verify не нужна отдельная соль.

`hash_password` и `verify_password` вычислительно тяжёлые. В async service их следует переносить в `tokio::task::spawn_blocking`, не блокируя worker. Подберите параметры по целевой машине; дефолты не являются универсальной production-политикой.

`verify_password` возвращает `Ok(false)` для неверного пароля. Повреждённая PHC-строка — ошибка формата/хранения, а не просто неверный пароль.

## Одинаковая ошибка credentials

При login неизвестный email и неверный пароль возвращают одинаковый `InvalidCredentials`. Это уменьшает возможность перебора зарегистрированных адресов. Точная причина остаётся во внутренней диагностике без пароля.

Регистрация сообщает занятый email по учебному контракту. В реальной системе также рассматривают защиту от account enumeration и подтверждение адреса.

## Access JWT

JWT содержит подписанные claims: `sub`, `role`, `iss`, `aud`, `exp`, `iat`, `jti`. Подпись защищает целостность, но payload не зашифрован. Не помещайте пароль и секретные данные в claims.

В учебном single-service используется HS256 с секретом не короче 32 байт. Алгоритм должен быть явно закреплён при decode, нельзя выбирать его только из JWT header. Issuer и audience проверяются в уроке 07.

Access token короткоживущий. Его stateless-природа ограничивает мгновенный отзыв: logout отзывает refresh, а уже выданный access естественно истекает. Для немедленного отзыва нужна дополнительная политика/denylist с её ценой.

## Opaque refresh token

Refresh token — случайная непрозрачная строка, не JWT. Клиент получает оригинал, сервер хранит только SHA-256 hash для lookup. SHA-256 здесь уместен, потому что случайный token имеет высокую энтропию, в отличие от пароля.

В заготовке источник случайности — UUID v4; для production можно использовать отдельный CSPRNG и закодировать не менее 256 бит. Не используйте короткие последовательные ID.

Refresh TTL больше access TTL, но оба положительны. При `refresh` старая запись помечается revoked, новая создаётся. Старый token больше не действует.

## Rotation, reuse и logout

Повторное использование revoked token — `RefreshReused`. В production это сигнал возможной кражи; часто отзывают всю token family и требуют повторный login. В учебной модели достаточно зафиксировать отказ.

Операция rotation должна быть атомарной: два одновременных refresh не должны оба успешно выпустить следующее поколение. In-memory lock охватывает проверку и замену; PostgreSQL использует транзакцию и блокировку строки/условный update.

Logout помечает token revoked и идемпотентен даже для неизвестного значения. Это не раскрывает, существовал ли token, и упрощает повторный logout.

## Время и тестирование

Методы login/refresh принимают `now` в Unix seconds. Это позволяет тестировать истечение без sleep и зависимости от системных часов. В production handler получает время надёжных часов.

При `now >= expires_at` token считается недействительным. Равенство уже означает истечение. Access `exp` имеет тип usize для JWT claims, а вычисления до проверенного диапазона выполняются в i64.

Не логируйте TokenPair, password hash, plaintext password или JWT secret. Ручная реализация `Debug` для AuthConfig показывает только `[REDACTED]`.

## PostgreSQL в итоговом проекте

Users table хранит email, password_hash и role. Refresh table обычно содержит token_hash, user_id, expiry, revoked и timestamps. Уникальный email проверяется и БД: проверка перед вставкой сама по себе не устраняет race.

Password hash не входит в публичный UserResponse. Refresh token не записывается plaintext в БД и логи. Backup БД содержит hashes и требует соответствующей защиты.

## Дополнительные production-меры

TLS обязателен, rate limiting на login/register, политика паролей и breached-password проверка по требованиям, secure secret management, мониторинг reuse, ротация ключей JWT и защита от brute force. Не добавляйте самодельную криптографию.

## Типичные ошибки

- SHA-256 как единственный password hash;
- разные ошибки «нет пользователя»/«неверный пароль»;
- бессрочный access JWT;
- plaintext refresh в БД;
- ротация без атомарности;
- принятие role при public registration;
- Argon2 внутри lock или async worker без spawn_blocking.

## Практика

[Домашнее задание](homework.md) задаёт контракты Argon2, registration/login/refresh/logout. В уроке 07 JWT будет проверен и превращён в CurrentUser middleware.

## Источники

- [RustCrypto: argon2](https://docs.rs/argon2/)
- [OWASP: Password Storage Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Password_Storage_Cheat_Sheet.html)
- [RFC 8725: JWT Best Current Practices](https://www.rfc-editor.org/rfc/rfc8725)
- [jsonwebtoken](https://docs.rs/jsonwebtoken/)
- [OWASP: JSON Web Token Security](https://cheatsheetseries.owasp.org/cheatsheets/JSON_Web_Token_for_Java_Cheat_Sheet.html)
