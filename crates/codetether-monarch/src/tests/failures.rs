//! Remote failures must not leak bodies or replay authenticated requests.
use super::request::read_request;
use crate::client::Client;
use tokio::{io::AsyncWriteExt, net::TcpListener};

#[tokio::test]
async fn unauthorized_and_redirect_are_not_retried_or_leaked() {
    for status in ["401 Unauthorized", "302 Found", "429 Too Many Requests"] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint = format!("http://{}/mcp", listener.local_addr().unwrap());
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            read_request(&mut stream).await;
            let reply = format!(
                "HTTP/1.1 {status}\r\nLocation: http://127.0.0.1:1/leak\r\nContent-Length: 12\r\nConnection: close\r\n\r\nprivate data"
            );
            stream.write_all(reply.as_bytes()).await.unwrap();
        });
        let mut client = Client::new(endpoint, "fixture-only".into()).unwrap();
        let error = client.initialize().await.unwrap_err().to_string();
        assert!(error.contains(&status[..3]) || error.contains("authorization expired"));
        assert!(!error.contains("private data"));
        assert!(!error.contains("fixture-only"));
        server.await.unwrap();
    }
}
