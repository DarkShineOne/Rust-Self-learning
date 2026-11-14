use std::time::Duration;

use aws_sdk_s3::config::{Credentials, Region};
use bytes::Bytes;
use uuid::Uuid;
use web_14_final_feedback::domain::AttachmentUpload;
use web_14_final_feedback::storage::{ObjectStorage, S3Storage};

#[tokio::test]
#[ignore = "requires a real self-hosted S3 endpoint, bucket and credentials"]
async fn s3_storage_uploads_presigns_and_deletes() {
    let endpoint = std::env::var("S3_TEST_ENDPOINT").expect("S3_TEST_ENDPOINT");
    let region = std::env::var("S3_TEST_REGION").expect("S3_TEST_REGION");
    let bucket = std::env::var("S3_TEST_BUCKET").expect("S3_TEST_BUCKET");
    let access_key = std::env::var("S3_TEST_ACCESS_KEY_ID").expect("S3_TEST_ACCESS_KEY_ID");
    let secret_key = std::env::var("S3_TEST_SECRET_ACCESS_KEY").expect("S3_TEST_SECRET_ACCESS_KEY");
    let credentials = Credentials::new(access_key, secret_key, None, None, "final-s3-test");
    let shared = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(region))
        .credentials_provider(credentials)
        .load()
        .await;
    let config = aws_sdk_s3::config::Builder::from(&shared)
        .endpoint_url(endpoint)
        .force_path_style(true)
        .build();
    let storage = S3Storage::new(aws_sdk_s3::Client::from_conf(config), bucket).unwrap();
    let feedback_id = Uuid::new_v4();
    let attachment_id = Uuid::new_v4();
    let upload = AttachmentUpload {
        file_name: "final.txt".into(),
        content_type: "text/plain".into(),
        bytes: Bytes::from_static(b"final integration"),
        max_bytes: 1024,
    };
    let key = S3Storage::object_key(feedback_id, attachment_id, &upload.file_name).unwrap();
    let attachment = storage
        .put(feedback_id, attachment_id, upload)
        .await
        .unwrap();
    assert_eq!(attachment.size_bytes, 17);
    let url = storage
        .download_url(&key, Duration::from_secs(60))
        .await
        .unwrap();
    assert!(!url.is_empty());
    storage.delete(&key).await.unwrap();
}
