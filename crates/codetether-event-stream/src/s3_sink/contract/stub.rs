//! stub regression.
use super::*;
use std::path::PathBuf;

#[tokio::test]
async fn stub_urls_and_invalid_filename() {
    for (endpoint, expected) in [
        (None, "s3://bucket/events/run/x"),
        (
            Some("http://localhost/root/".into()),
            "http://localhost/root/events/run/x",
        ),
    ] {
        let sink = S3Sink::from_config(config(endpoint, true)).await.unwrap();
        assert_eq!(
            sink.upload_bytes(b"hello", "events/run/x", "text/plain")
                .await
                .unwrap(),
            expected
        );
        assert_eq!(sink.bucket_name(), "bucket");
        assert_eq!(sink.prefix(), "events/");
        assert!(matches!(
            sink.upload_file(&PathBuf::from("/"), "run").await,
            Err(S3SinkError::UploadFailed(_))
        ));
    }
}
