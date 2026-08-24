//! Redirect confinement for the WebSearch HTTP client.

#[tokio::test]
async fn search_client_does_not_follow_redirects() {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut request = [0; 512];
        let _ = socket.read(&mut request).await.unwrap();
        socket
            .write_all(
                format!(
                    "HTTP/1.1 302 Found\r\nLocation: http://{address}/retargeted\r\nContent-Length: 0\r\n\r\n"
                )
                .as_bytes(),
            )
            .await
            .unwrap();
    });
    let response = super::build()
        .get(format!("http://{address}/search"))
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::FOUND);
    server.await.unwrap();
}
