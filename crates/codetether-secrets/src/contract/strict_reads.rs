//! Strict lifecycle reads distinguish missing grants from denied backend access.
use super::{fixture::Fixture, response};
use serde_json::json;

#[tokio::test]
async fn strict_provider_reads_preserve_backend_failures() {
    let server = Fixture::new(vec![
        (
            200,
            response::read(json!({"monarch_oauth":{"scope":"mcp:read"}})),
        ),
        (404, response::error()),
        (403, response::error()),
        // The existing manager retries once after a rejected Vault token.
        (403, response::error()),
    ])
    .await;
    let manager = server.manager().await;
    let record = manager
        .read_provider_secrets("monarch")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(record.extra["monarch_oauth"]["scope"], "mcp:read");
    assert!(
        manager
            .read_provider_secrets("monarch")
            .await
            .unwrap()
            .is_none()
    );
    let error = manager
        .read_provider_secrets("monarch")
        .await
        .unwrap_err()
        .to_string();
    assert_eq!(error, "Vault provider record read failed");
    assert!(!error.contains("fixture rejection"));
    let calls = server.calls.lock().unwrap();
    assert_eq!(calls.requests.len(), 4);
    assert!(
        calls
            .requests
            .iter()
            .all(|(_, path, _)| path == "/v1/secret/data/codetether/providers/monarch")
    );
}
