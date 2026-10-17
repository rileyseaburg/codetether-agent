//! Chunked responses are checked while reading, without trusting headers.

use super::*;
#[tokio::test]
async fn rejects_chunked_body_above_cap() {
    let response = response("HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n4\r\nabcd\r\n4\r\nefgh\r\n0\r\n\r\n").await;
    let error = read_body_capped(response, 4).await.unwrap_err();
    assert!(error.to_string().contains("exceeded cap"));
}
