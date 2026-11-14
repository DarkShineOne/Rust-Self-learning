# Домашнее задание: S3Storage и согласованность

## Валидация и keys

```rust
pub fn validate_upload(input: &PutObject) -> Result<(), StorageError>;
pub fn S3Storage::new(client: aws_sdk_s3::Client, bucket: String)
    -> Result<Self, StorageError>;
pub fn S3Storage::object_key(feedback_id: Uuid, attachment_id: Uuid, file_name: &str)
    -> Result<String, StorageError>;
```

- Порядок validate: пустой bytes → `EmptyFile`; `max_bytes == 0` или `bytes.len() > max_bytes` → `FileTooLarge`; невалидное имя → `InvalidFileName`; MIME вне allowlist → `UnsupportedContentType`.
- Разрешены `text/plain`, `image/png`, `image/jpeg`, `application/pdf`.
- Длина имени после trim — от 1 до 255 символов `.chars().count()`; запрещены `/`, `\\` и control characters. Сохраняется trimmed.
- `new`: trimmed bucket непустой, иначе `InvalidBucket`.
- Key: `feedback/{feedback_id}/{attachment_id}/{trimmed_file_name}`.
- Для download/delete key должен начинаться с `feedback/`, не содержать `..`, `\\` и control characters, иначе `InvalidKey`.

## ObjectStorage

Реализуйте методы трейта для `S3Storage`:

```rust
async fn put(&self, input: PutObject) -> Result<AttachmentMetadata, StorageError>;
async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, StorageError>;
async fn delete(&self, key: &str) -> Result<(), StorageError>;
```

- `put`: validate, object_key, `ByteStream::from(input.bytes)`, `put_object` с bucket/key/content_type. Вернуть metadata с исходными UUID, trimmed original name и размером в i64.
- `download_url`: validate key; TTL от 1 до 3600 секунд включительно, иначе `InvalidTtl`; `get_object(...).presign(PresigningConfig)` и вернуть URL.
- `delete`: validate key и `delete_object`.
- SDK/конфигурационные ошибки → `Service(message)`. Не логируйте credentials или presigned URL.

## AttachmentService

```rust
pub fn AttachmentService::new(storage: S, metadata: M) -> Self;
pub async fn upload(&self, input: PutObject)
    -> Result<AttachmentMetadata, StorageError>;
```

- Сначала validate, затем storage.put, затем metadata.save.
- При ошибке storage.put не вызывать metadata.save и delete.
- При ошибке metadata.save один раз попытаться `storage.delete(key)`. Ошибка cleanup не заменяет исходную ошибку metadata; в рабочем коде её следует логировать без секретов.
- При успехе вернуть metadata.

## Проверка без S3

```bash
cargo test -p web-10-s3 --tests
cargo clippy -p web-10-s3 --all-targets -- -D warnings
```

## Настоящий S3 integration test

Самостоятельно установите open-source S3-совместимое хранилище, создайте bucket и задайте:

```bash
export S3_TEST_ENDPOINT="http://127.0.0.1:9000"
export S3_TEST_REGION="us-east-1"
export S3_TEST_BUCKET="feedback-test"
export S3_TEST_ACCESS_KEY_ID="your-test-key"
export S3_TEST_SECRET_ACCESS_KEY="your-test-secret"
cargo test -p web-10-s3 --test s3_integration -- --ignored
```

На PowerShell используйте `$env:S3_TEST_ENDPOINT` и аналогичные переменные. Bucket должен существовать. Тест создаёт уникальный key и удаляет объект; не используйте production bucket.
