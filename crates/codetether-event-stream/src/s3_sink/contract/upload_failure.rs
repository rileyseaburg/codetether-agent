//! upload failure regression.
use super::*;

#[tokio::test]
async fn failed_upload_preserves_status_and_body() {
    let (endpoint, server) = super::server::start("403 Forbidden");
    let sink = S3Sink::from_config(config(Some(endpoint), false))
        .await
        .unwrap();
    let error = sink
        .upload_bytes(b"hello", "x", "text/plain")
        .await
        .unwrap_err();
    assert!(
        matches!(error, S3SinkError::S3Error(ref message) if message == "403 Forbidden: result")
    );
    server.join().unwrap();
}
