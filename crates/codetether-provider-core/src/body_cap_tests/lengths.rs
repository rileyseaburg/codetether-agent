//! Content-Length checks and an exact-limit JSON body.

use super::*;
#[tokio::test]
async fn rejects_declared_length_above_cap() {
    let response =
        response("HTTP/1.1 200 OK\r\nContent-Length: 6\r\nConnection: close\r\n\r\nabcdef").await;
    let error = read_body_capped(response, 4).await.unwrap_err();
    assert!(error.to_string().contains("Content-Length"));
}
#[tokio::test]
async fn accepts_json_at_exact_cap() {
    let response =
        response("HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nnull").await;
    let value: serde_json::Value = json_capped(response, 4).await.unwrap();
    assert!(value.is_null());
}
