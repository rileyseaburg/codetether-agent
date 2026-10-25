//! Local-only KV-v2 fixture; no real Vault or AWS credentials are used.
use codetether_secrets::{SecretsManager, VaultConfig};
use serde_json::json;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

pub(super) async fn start() -> (
    SecretsManager,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
    tokio::task::JoinHandle<()>,
) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let version = Arc::new(AtomicUsize::new(0));
    let reads = Arc::new(AtomicUsize::new(0));
    let (current, calls) = (version.clone(), reads.clone());
    let server = tokio::spawn(async move {
        loop {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            while !request.windows(4).any(|part| part == b"\r\n\r\n") {
                let mut buf = [0; 1024];
                let n = socket.read(&mut buf).await.unwrap();
                assert!(n > 0);
                request.extend_from_slice(&buf[..n]);
            }
            let request = String::from_utf8(request).unwrap();
            let target = request.split_whitespace().nth(1).unwrap();
            let path = target.split('?').next().unwrap();
            let (status, body) = if path == "/v1/secret/data/codetether/providers/bedrock" {
                assert!(request.starts_with("GET "));
                calls.fetch_add(1, Ordering::SeqCst);
                if current.load(Ordering::SeqCst) == usize::MAX {
                    std::future::pending::<()>().await;
                }
                (
                    "200 OK",
                    json!({"lease_id":"", "lease_duration":0, "renewable":false, "request_id":"fixture",
                    "data":{"data":{"api_key":format!("fixture-{}", current.load(Ordering::SeqCst))},
                    "metadata":{"created_time":"2026-01-01T00:00:00Z","deletion_time":"","destroyed":false,"version":1}}}),
                )
            } else {
                // Lease maintenance is independent of provider-secret reads.
                assert!(
                    path.starts_with("/v1/auth/token/"),
                    "unexpected fixture path: {path}"
                );
                (
                    "403 Forbidden",
                    json!({"errors":["fixture has no token-maintenance capability"]}),
                )
            };
            let body = body.to_string();
            let reply = format!(
                "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(reply.as_bytes()).await.unwrap();
        }
    });
    let manager = SecretsManager::new(&VaultConfig {
        address,
        token: "fixture-token".into(),
        mount: None,
        path: None,
    })
    .await
    .unwrap();
    (manager, version, reads, server)
}
