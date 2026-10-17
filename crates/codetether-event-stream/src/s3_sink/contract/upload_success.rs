//! upload success regression.
use super::*;
use crate::s3_sink::digest::sha256_hex_bytes;

#[tokio::test]
async fn successful_upload_preserves_wire_and_return_url() {
    let (endpoint, server) = super::server::start("200 OK");
    let sink = S3Sink::from_config(config(Some(endpoint.clone()), false))
        .await
        .unwrap();
    let url = sink
        .upload_bytes(b"hello", "events/run/x", "text/plain")
        .await
        .unwrap();
    assert_eq!(url, format!("{endpoint}/events/run/x"));
    let request = server.join().unwrap();
    assert!(request.starts_with("PUT /bucket/events/run/x HTTP/1.1\r\n"));
    assert!(request.contains("content-type: application/json\r\n"));
    assert!(request.contains(&format!(
        "x-amz-content-sha256: {}\r\n",
        sha256_hex_bytes(b"hello")
    )));
    assert!(request.contains("authorization: AWS4-HMAC-SHA256 Credential=fixture-access/"));
    assert!(request.ends_with("\r\n\r\nhello"));
}
