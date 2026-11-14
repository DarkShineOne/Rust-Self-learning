use std::sync::Mutex;
use std::time::Duration;

use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;
use web_10_s3::{
    AttachmentMetadata, AttachmentMetadataStore, AttachmentService, ObjectStorage, PutObject,
    S3Storage, StorageError, validate_upload,
};

fn input() -> PutObject {
    PutObject {
        feedback_id: Uuid::new_v4(),
        attachment_id: Uuid::new_v4(),
        file_name: "  note.txt  ".into(),
        content_type: "text/plain".into(),
        bytes: Bytes::from_static(b"feedback"),
        max_bytes: 100,
    }
}

#[test]
fn validates_upload_and_builds_safe_object_key() {
    assert_eq!(validate_upload(&input()), Ok(()));
    let value = input();
    assert_eq!(
        S3Storage::object_key(value.feedback_id, value.attachment_id, &value.file_name).unwrap(),
        format!(
            "feedback/{}/{}/note.txt",
            value.feedback_id, value.attachment_id
        )
    );

    let mut invalid = input();
    invalid.file_name = "../secret.txt".into();
    assert_eq!(
        validate_upload(&invalid),
        Err(StorageError::InvalidFileName)
    );
    invalid.file_name = "note.txt".into();
    invalid.content_type = "application/x-msdownload".into();
    assert_eq!(
        validate_upload(&invalid),
        Err(StorageError::UnsupportedContentType)
    );
}

#[test]
fn rejects_empty_and_oversized_files() {
    let mut value = input();
    value.bytes = Bytes::new();
    assert_eq!(validate_upload(&value), Err(StorageError::EmptyFile));
    value.bytes = Bytes::from_static(b"too large");
    value.max_bytes = 2;
    assert_eq!(validate_upload(&value), Err(StorageError::FileTooLarge));
}

#[derive(Default)]
struct StorageState {
    puts: usize,
    deletes: Vec<String>,
}

#[derive(Clone, Default)]
struct MockStorage {
    state: std::sync::Arc<Mutex<StorageState>>,
    fail_put: bool,
}

#[async_trait]
impl ObjectStorage for MockStorage {
    async fn put(&self, input: PutObject) -> Result<AttachmentMetadata, StorageError> {
        self.state.lock().unwrap().puts += 1;
        if self.fail_put {
            return Err(StorageError::Service("s3 unavailable".into()));
        }
        Ok(AttachmentMetadata {
            id: input.attachment_id,
            feedback_id: input.feedback_id,
            key: format!(
                "feedback/{}/{}/note.txt",
                input.feedback_id, input.attachment_id
            ),
            original_name: input.file_name.trim().into(),
            content_type: input.content_type,
            size_bytes: input.bytes.len() as i64,
        })
    }

    async fn download_url(&self, key: &str, _ttl: Duration) -> Result<String, StorageError> {
        Ok(format!("https://storage.test/{key}"))
    }

    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        self.state.lock().unwrap().deletes.push(key.into());
        Ok(())
    }
}

#[derive(Clone, Default)]
struct MockMetadata {
    saved: std::sync::Arc<Mutex<Vec<AttachmentMetadata>>>,
    fail: bool,
}

#[async_trait]
impl AttachmentMetadataStore for MockMetadata {
    async fn save(&self, metadata: AttachmentMetadata) -> Result<(), StorageError> {
        if self.fail {
            return Err(StorageError::Service("database unavailable".into()));
        }
        self.saved.lock().unwrap().push(metadata);
        Ok(())
    }
}

#[tokio::test]
async fn successful_upload_saves_metadata_without_cleanup() {
    let storage = MockStorage::default();
    let metadata = MockMetadata::default();
    let service = AttachmentService::new(storage.clone(), metadata.clone());
    let result = service.upload(input()).await.unwrap();
    assert_eq!(result.original_name, "note.txt");
    assert_eq!(metadata.saved.lock().unwrap().len(), 1);
    assert!(storage.state.lock().unwrap().deletes.is_empty());
}

#[tokio::test]
async fn metadata_failure_compensates_uploaded_object() {
    let storage = MockStorage::default();
    let metadata = MockMetadata {
        fail: true,
        ..MockMetadata::default()
    };
    let service = AttachmentService::new(storage.clone(), metadata);
    assert_eq!(
        service.upload(input()).await,
        Err(StorageError::Service("database unavailable".into()))
    );
    assert_eq!(storage.state.lock().unwrap().deletes.len(), 1);
}

#[tokio::test]
async fn storage_failure_does_not_touch_metadata_or_cleanup() {
    let storage = MockStorage {
        fail_put: true,
        ..MockStorage::default()
    };
    let metadata = MockMetadata::default();
    let service = AttachmentService::new(storage.clone(), metadata.clone());
    assert_eq!(
        service.upload(input()).await,
        Err(StorageError::Service("s3 unavailable".into()))
    );
    assert!(metadata.saved.lock().unwrap().is_empty());
    assert!(storage.state.lock().unwrap().deletes.is_empty());
}
