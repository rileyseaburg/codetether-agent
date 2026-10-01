//! Fixture-backed protocol boundary and output-limit tests.

use super::super::response;
use serde_json::json;

#[tokio::test]
async fn rustyroad_skips_diagnostics_without_echoing_them() {
    let mut input =
        &b"private diagnostic\n{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"ok\":true}}\n"[..];
    assert_eq!(
        response::read(&mut input, 1).await.unwrap(),
        json!({"ok":true})
    );
}

#[tokio::test]
async fn rustyroad_rejects_rpc_errors_mismatched_ids_and_eof() {
    for line in [
        "{\"jsonrpc\":\"2.0\",\"id\":2,\"result\":{}}\n",
        "{\"jsonrpc\":\"1.0\",\"id\":1,\"result\":{}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":1,\"error\":{\"code\":-1,\"message\":\"fixture error\"}}\n",
        "{\"jsonrpc\":\"2.0\",\"id\":1}\n",
        "",
    ] {
        assert!(response::read(&mut line.as_bytes(), 1).await.is_err());
    }
}

#[tokio::test]
async fn rustyroad_bounds_output_and_non_response_lines() {
    let bytes = vec![b'x'; 1_048_577];
    assert!(
        response::read(&mut bytes.as_slice(), 1)
            .await
            .unwrap_err()
            .to_string()
            .contains("1 MiB")
    );
    let lines = "diagnostic\n".repeat(32);
    assert!(
        response::read(&mut lines.as_bytes(), 1)
            .await
            .unwrap_err()
            .to_string()
            .contains("too many")
    );
}
