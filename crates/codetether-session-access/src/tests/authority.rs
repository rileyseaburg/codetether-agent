//! Fail-closed authority regressions against a mocked loopback server.
use super::super::{authority::workspace, http_error};
use super::fixture::{authority, headers};
use axum::http::{HeaderMap, StatusCode};

mod input;

#[tokio::test]
async fn forwards_only_caller_credentials_and_accepts_matching_binding() {
    let dir = tempfile::tempdir().unwrap();
    let body = serde_json::json!({"id": "workspace", "path": dir.path()}).to_string();
    let mut server = authority(StatusCode::OK, &body).await;
    let url = format!("{}/?ignored=true#ignored", server.url);
    assert_eq!(
        workspace(&url, "workspace", &headers()).await.unwrap(),
        dir.path()
    );
    let (uri, forwarded) = server.requests.recv().await.unwrap();
    assert_eq!(uri.path(), "/v1/agent/workspaces/workspace/session-access");
    assert!(uri.query().is_none());
    assert_eq!(forwarded["authorization"], "Bearer test-caller");
}

#[tokio::test]
async fn rejects_authority_denials_redirects_and_server_failures() {
    for (input, expected) in [(401, 401), (403, 403), (404, 403), (302, 503), (500, 503)] {
        let server = authority(StatusCode::from_u16(input).unwrap(), "rejected").await;
        let error = workspace(&server.url, "workspace", &headers())
            .await
            .unwrap_err();
        assert_eq!(http_error(error).0.as_u16(), expected);
    }
}

#[tokio::test]
async fn rejects_untrusted_or_malformed_bindings() {
    for (body, expected) in [
        (r#"{"id":"other","path":"/tmp"}"#, 403),
        (r#"{"id":"workspace","path":"relative"}"#, 403),
        (r#"{"id":"workspace"}"#, 503),
        ("not-json", 503),
    ] {
        let server = authority(StatusCode::OK, body).await;
        let error = workspace(&server.url, "workspace", &headers())
            .await
            .unwrap_err();
        assert_eq!(http_error(error).0.as_u16(), expected);
    }
}
