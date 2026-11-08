//! Exercise initialization, incremental SSE replies, and session cleanup.
use super::server::serve;
use crate::client::Client;
use serde_json::json;
use tokio::net::TcpListener;

#[tokio::test]
async fn json_initialize_sse_response_and_session_cleanup() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
    let server = tokio::spawn(serve(listener));
    let mut client = Client::new(endpoint, "fixture-only".into()).unwrap();
    client.initialize().await.unwrap();
    assert_eq!(
        client.rpc("tools/list", json!({})).await.unwrap(),
        json!({"tools":[]})
    );
    client.close().await;
    server.await.unwrap();
}
