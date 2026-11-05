//! Refresh rotation retains omitted tokens and refuses broader scopes.
use super::server::serve;
use crate::oauth::{credentials::Credentials, refresh::refresh_at};
use reqwest::Client;
use tokio::net::TcpListener;

#[tokio::test]
async fn mock_refresh_rotates_preserves_and_rejects_scope_escalation() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/token", listener.local_addr().unwrap());
    let server = tokio::spawn(serve(listener));
    let old = Credentials {
        client_id: "client".into(),
        access_token: "old".into(),
        refresh_token: Some("original-refresh".into()),
        expires_at: 0,
        scope: "mcp:read".into(),
    };
    let client = Client::builder()
        .no_proxy()
        .timeout(std::time::Duration::from_secs(5))
        .build()
        .unwrap();
    let rotated = refresh_at(&client, &old, &endpoint).await.unwrap();
    assert_eq!(rotated.refresh_token.as_deref(), Some("rotated-refresh"));
    assert!(rotated.fresh().unwrap());
    let retained = refresh_at(&client, &rotated, &endpoint).await.unwrap();
    assert_eq!(retained.refresh_token.as_deref(), Some("rotated-refresh"));
    assert!(refresh_at(&client, &retained, &endpoint).await.is_err());
    server.await.unwrap();
}
