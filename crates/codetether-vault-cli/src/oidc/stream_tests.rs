//! OIDC subprocess protocol: capture credentials rather than emitting them.

#[tokio::test]
async fn oidc_stream_collects_json_without_returning_progress() {
    let bytes = b"Complete browser login\nhttps://issuer.example/authorize?state=fixture\n{\"auth\":{\"client_token\":\"fixture-token\"}}\n";
    let output = super::collect(&bytes[..], true).await.unwrap();
    assert!(output.starts_with('{'));
    assert!(!output.contains("Complete browser login"));
}
