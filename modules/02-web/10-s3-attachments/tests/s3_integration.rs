#![allow(clippy::expect_used)]

use std::time::Duration;

use aws_sdk_s3::config::{Credentials, Region};
use bytes::Bytes;
use uuid::Uuid;
use web_10_s3::{ObjectStorage, PutObject, S3Storage};

#[tokio::test]
#[ignore = "requires a real S3 endpoint, bucket and credentials"]
async fn uploads_presigns_and_deletes_real_s3_object() {
    let endpoint = std::env::var("S3_TEST_ENDPOINT").expect("S3_TEST_ENDPOINT");
    let region = std::env::var("S3_TEST_REGION").expect("S3_TEST_REGION");
    let bucket = std::env::var("S3_TEST_BUCKET").expect("S3_TEST_BUCKET");
    let access_key = std::env::var("S3_TEST_ACCESS_KEY_ID").expect("S3_TEST_ACCESS_KEY_ID");
    let secret_key = std::env::var("S3_TEST_SECRET_ACCESS_KEY").expect("S3_TEST_SECRET_ACCESS_KEY");

    let credentials = Credentials::new(access_key, secret_key, None, None, "s3-integration-test");
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
    let input = PutObject {
        feedback_id: Uuid::new_v4(),
        attachment_id: Uuid::new_v4(),
        file_name: "integration.txt".into(),
        content_type: "text/plain".into(),
        bytes: Bytes::from_static(b"real S3 integration"),
        max_bytes: 1024,
    };
    let key =
        S3Storage::object_key(input.feedback_id, input.attachment_id, &input.file_name).unwrap();

    let metadata = storage.put(input).await.unwrap();
    assert_eq!(metadata.size_bytes, 19);
    let url = storage
        .download_url(&key, Duration::from_secs(60))
        .await
        .unwrap();
    assert!(!url.is_empty());
    storage.delete(&key).await.unwrap();
}
