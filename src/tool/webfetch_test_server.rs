//! One-request loopback server for WebFetch authorization tests.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

pub(super) async fn start() -> (String, Arc<AtomicUsize>, tokio::task::JoinHandle<()>) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener");
    let url = format!("http://{}/proof", listener.local_addr().expect("address"));
    let hits = Arc::new(AtomicUsize::new(0));
    let server_hits = Arc::clone(&hits);
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.expect("accept");
        server_hits.fetch_add(1, Ordering::SeqCst);
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let mut request = [0_u8; 1024];
        let _ = socket.read(&mut request).await.expect("read");
        socket
            .write_all(
                b"HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 5\r\nConnection: close\r\n\r\nproof",
            )
            .await
            .expect("write");
    });
    (url, hits, server)
}

pub(super) fn count(hits: &AtomicUsize) -> usize {
    hits.load(Ordering::SeqCst)
}
