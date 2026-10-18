//! End-to-end test for A2A RPC bearer auth.
//!
//! Lives in its own test binary because it sets `CODETETHER_AUTH_TOKEN`,
//! which is process-global and would poison parallel tests sharing
//! the env. The `OnceLock` cache in `server_auth.rs` makes the token
//! stable for the rest of the binary's lifetime.

use codetether_agent::a2a::server::A2AServer;
use serde_json::json;

#[tokio::test]
async fn rpc_requires_bearer_when_token_set() {
    unsafe { std::env::set_var("CODETETHER_AUTH_TOKEN", "test-secret") };

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let router = A2AServer::new(A2AServer::default_card(&url)).router();
    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    let client = reqwest::Client::new();
    let rpc_body = json!({
        "jsonrpc": "2.0", "id": 1, "method": "tasks/get",
        "params": {"id": "nope"}
    });

    // 1. No token → 401.
    let r = client.post(&url).json(&rpc_body).send().await.unwrap();
    assert_eq!(r.status(), 401, "missing bearer must be rejected");

    // 2. Wrong token → 401.
    let r = client
        .post(&url)
        .bearer_auth("wrong")
        .json(&rpc_body)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 401, "wrong bearer must be rejected");

    // 3. Correct token → JSON-RPC layer reached (200 envelope).
    let r = client
        .post(&url)
        .bearer_auth("test-secret")
        .json(&rpc_body)
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 200, "valid bearer must pass auth");

    // 4. Agent card stays public even with auth enabled.
    let r = client
        .get(format!("{url}/.well-known/agent.json"))
        .send()
        .await
        .unwrap();
    assert_eq!(r.status(), 200, "discovery must remain public");
}

#[test]
fn constant_time_eq_does_not_leak_length() {
    use codetether_agent::a2a::server_auth::constant_time_eq as ct;
    assert!(ct(b"hello", b"hello"));
    assert!(!ct(b"hello", b"world"));
    assert!(!ct(b"short", b"longer-token"));
    assert!(!ct(b"a", b""));
}
