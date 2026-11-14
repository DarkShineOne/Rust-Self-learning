#![allow(unused_variables, dead_code)]

use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PutObject {
    pub feedback_id: Uuid,
    pub attachment_id: Uuid,
    pub file_name: String,
    pub content_type: String,
    pub bytes: Bytes,
    pub max_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttachmentMetadata {
    pub id: Uuid,
    pub feedback_id: Uuid,
    pub key: String,
    pub original_name: String,
    pub content_type: String,
    pub size_bytes: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum StorageError {
    #[error("file must not be empty")]
    EmptyFile,
    #[error("file exceeds the configured size limit")]
    FileTooLarge,
    #[error("invalid file name")]
    InvalidFileName,
    #[error("unsupported content type")]
    UnsupportedContentType,
    #[error("invalid object key")]
    InvalidKey,
    #[error("invalid bucket")]
    InvalidBucket,
    #[error("invalid presigned URL TTL")]
    InvalidTtl,
    #[error("storage service error: {0}")]
    Service(String),
}

/// Проверяет размер, имя и MIME allowlist до обращения к S3.
pub fn validate_upload(input: &PutObject) -> Result<(), StorageError> {
    todo!("проверить empty/max/path/control/MIME")
}

#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put(&self, input: PutObject) -> Result<AttachmentMetadata, StorageError>;
    async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, StorageError>;
    async fn delete(&self, key: &str) -> Result<(), StorageError>;
}

#[derive(Debug, Clone)]
pub struct S3Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3Storage {
    pub fn new(client: aws_sdk_s3::Client, bucket: String) -> Result<Self, StorageError> {
        todo!("bucket после trim непустой")
    }

    /// Создаёт feedback/{feedback_id}/{attachment_id}/{safe_file_name}.
    pub fn object_key(
        feedback_id: Uuid,
        attachment_id: Uuid,
        file_name: &str,
    ) -> Result<String, StorageError> {
        todo!("отклонить separators/control и непустое trimmed имя")
    }
}

#[async_trait]
impl ObjectStorage for S3Storage {
    async fn put(&self, input: PutObject) -> Result<AttachmentMetadata, StorageError> {
        todo!("ByteStream + put_object bucket/key/content_type")
    }

    async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, StorageError> {
        todo!("PresigningConfig + get_object.presign, TTL 1..=3600 секунд")
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        todo!("проверить key и delete_object")
    }
}

#[async_trait]
pub trait AttachmentMetadataStore: Send + Sync {
    async fn save(&self, metadata: AttachmentMetadata) -> Result<(), StorageError>;
}

/// Координирует S3 и метаданные без ложной распределённой транзакции.
pub struct AttachmentService<S, M> {
    storage: S,
    metadata: M,
}

impl<S: ObjectStorage, M: AttachmentMetadataStore> AttachmentService<S, M> {
    pub fn new(storage: S, metadata: M) -> Self {
        todo!("сохранить зависимости")
    }

    /// При ошибке metadata.save пытается удалить уже загруженный S3 object.
    pub async fn upload(&self, input: PutObject) -> Result<AttachmentMetadata, StorageError> {
        todo!("validate → S3 put → DB save; compensating delete при ошибке DB")
    }
}
