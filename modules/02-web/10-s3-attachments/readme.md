# S3 и файловые вложения

## Цели

Научиться подключать S3-совместимое хранилище через официальный AWS SDK for Rust, проверять файлы, строить безопасные object keys, выдавать presigned URL и согласовывать S3 с PostgreSQL без иллюзии распределённой транзакции.

## Установка хранилища — задача разработчика

Курс не устанавливает S3 и не добавляет его в Compose. Для самостоятельной установки доступны open-source проекты:

- [MinIO](https://github.com/minio/minio) — S3-совместимое объектное хранилище;
- [SeaweedFS](https://github.com/seaweedfs/seaweedfs) — распределённое хранилище с S3 API;
- [Ceph RGW](https://docs.ceph.com/en/latest/radosgw/) — S3-совместимый gateway более крупной распределённой системы.

Выберите проект и следуйте его актуальной документации по установке на вашу ОС. Создайте отдельный bucket и credentials с минимально необходимыми правами. Не используйте административные ключи приложения в тестах.

Приложение зависит от S3 API, а не конкретного продукта. Endpoint, region, bucket, access key, secret key и path-style настройка передаются через окружение.

## Bucket, key и объект

Bucket — логический контейнер, key — полное имя объекта. S3 не является обычной файловой системой с каталогами: `feedback/.../file.png` — просто key с separators.

В PostgreSQL сохраняются metadata: attachment id, feedback id, key, original name, content type, size. Содержимое находится только в S3. Не храните постоянный публичный URL и не делайте bucket публичным.

Object key генерируется сервером из UUID. Original filename сохраняется отдельно для отображения, но не может содержать `/`, `\\`, управляющие символы или быть пустым после trim. Это предотвращает path-like ambiguity и неприятные keys.

## AWS SDK и self-hosted endpoint

`aws_sdk_s3::Client` выполняет подписанные S3-запросы. Для self-hosted сервера конфигурация обычно включает:

- custom `endpoint_url`;
- явный Region, часто `us-east-1` как значение конфигурации совместимости;
- static test credentials или provider chain;
- `force_path_style(true)`, если сервер не поддерживает virtual-host bucket addressing.

`ByteStream` передаёт тело `put_object`. В учебном контракте Bytes уже ограничены max_bytes; в реальном multipart handler следует читать stream с лимитом, а не сначала буферизовать неограниченный файл.

Не отключайте проверку TLS в production. Локальный HTTP endpoint допустим только в контролируемой учебной сети.

## Размер и content type

Allowlist урока: `text/plain`, `image/png`, `image/jpeg`, `application/pdf`. Пустой файл и превышение max_bytes отклоняются до S3.

`Content-Type` от клиента не является доказательством содержимого. В production требуются sniffing/magic bytes, антивирусная проверка по рискам, безопасная политика отображения и `Content-Disposition`. PDF и изображения не становятся безопасными только из-за MIME.

Ограничение нужно применять на уровне HTTP body/multipart и при чтении stream. Проверка `bytes.len()` после буферизации — последний рубеж, а не защита памяти от уже отправленного огромного тела.

## Upload

`S3Storage::put` выполняет validate, генерирует key, вызывает `put_object` с bucket/key/content_type/body и формирует AttachmentMetadata. `size_bytes` проверяется при преобразовании в i64; практический max должен быть значительно меньше предела типа.

Ошибка SDK преобразуется в `StorageError::Service` без credentials в сообщении. Не включайте access/secret keys в Debug или логи.

## Presigned URL

`GET` объекта выполняется через короткоживущий presigned URL. `PresigningConfig` задаёт TTL 1–3600 секунд по контракту. Подпись предоставляет доступ к конкретному key без постоянного публичного bucket.

Presigned URL — bearer credential: любой, кто получил его до истечения, может использовать его. Не пишите URL в логи и не передавайте третьим лицам. Срок действия не заменяет проверку прав перед выдачей.

`download_url` проверяет, что key начинается с `feedback/`, не содержит `..`, backslash и control characters. Delete использует ту же проверку key.

## Согласованность S3 и PostgreSQL

Нельзя атомарно закоммитить объект в S3 и строку PostgreSQL одной локальной транзакцией. `AttachmentService::upload` использует последовательность:

1. проверить вход;
2. загрузить объект;
3. сохранить metadata;
4. если шаг 3 неуспешен, попытаться удалить объект и вернуть исходную ошибку metadata.

Compensating delete снижает число orphaned objects, но тоже может не успеть. В production нужны периодический cleanup/reconciliation, lifecycle policies и журнал операций. Не утверждайте, что операция «полностью атомарна».

При удалении feedback также нужно определить порядок DB/S3, повторные попытки и поведение при сбое. Это включено в ревью итогового проекта.

## Тестовая стратегия

`ObjectStorage` и `AttachmentMetadataStore` — async traits. Unit tests используют hand-written mocks и проверяют вызовы: успешный save, отсутствие save после ошибки S3, compensating delete после ошибки metadata.

Отдельный `#[ignore]` integration test использует настоящий endpoint и заранее созданный bucket. Он создаёт UUID keys и удаляет объект. Для его запуска нужны переменные из домашнего задания; отсутствие S3 не должно незаметно превращаться в успешный локальный mock.

## Типичные ошибки

- доверять filename и Content-Type;
- неограниченно читать multipart в память;
- публичный bucket и вечный URL;
- plaintext credentials в репозитории/логах;
- хранить файл в PostgreSQL вопреки архитектуре;
- считать S3 + DB одной транзакцией;
- молча пропускать integration test при отсутствии конфигурации.

## Практика

[Домашнее задание](homework.md) содержит методы S3Storage, validate и координацию сервиса. Следующий урок систематизирует тестирование всех слоёв.

## Источники

- [AWS SDK for Rust: S3 Client](https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/struct.Client.html)
- [AWS SDK: Presigning](https://docs.rs/aws-sdk-s3/latest/aws_sdk_s3/presigning/index.html)
- [AWS S3 API: PutObject](https://docs.aws.amazon.com/AmazonS3/latest/API/API_PutObject.html)
- [OWASP: File Upload Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/File_Upload_Cheat_Sheet.html)
