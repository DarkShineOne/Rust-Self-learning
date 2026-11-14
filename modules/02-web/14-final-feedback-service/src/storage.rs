use std::time::Duration;

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::{AppError, Attachment, AttachmentUpload};

#[async_trait]
pub trait ObjectStorage: Send + Sync {
    async fn put(
        &self,
        feedback_id: Uuid,
        attachment_id: Uuid,
        upload: AttachmentUpload,
    ) -> Result<Attachment, AppError>;
    async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, AppError>;
    async fn delete(&self, key: &str) -> Result<(), AppError>;
}

pub fn validate_upload(upload: &AttachmentUpload) -> Result<(), AppError> {
    todo!("empty/max/name/MIME: text/plain,image/png,image/jpeg,application/pdf")
}

#[derive(Debug, Clone)]
pub struct S3Storage {
    client: aws_sdk_s3::Client,
    bucket: String,
}

impl S3Storage {
    pub fn new(client: aws_sdk_s3::Client, bucket: String) -> Result<Self, AppError> {
        todo!("проверить bucket")
    }

    pub fn object_key(
        feedback_id: Uuid,
        attachment_id: Uuid,
        file_name: &str,
    ) -> Result<String, AppError> {
        todo!("feedback/{{feedback_id}}/{{attachment_id}}/{{safe_name}}")
    }
}

#[async_trait]
impl ObjectStorage for S3Storage {
    async fn put(
        &self,
        feedback_id: Uuid,
        attachment_id: Uuid,
        upload: AttachmentUpload,
    ) -> Result<Attachment, AppError> {
        todo!("validate + ByteStream + put_object")
    }

    async fn download_url(&self, key: &str, ttl: Duration) -> Result<String, AppError> {
        todo!("validate key + presign 1..=3600 секунд")
    }

    async fn delete(&self, key: &str) -> Result<(), AppError> {
        todo!("validate key + delete_object")
    }
}
