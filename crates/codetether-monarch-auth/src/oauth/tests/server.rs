//! Serve deterministic token rotation and scope-escalation fixtures.
use super::request::read_request;
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub(super) async fn serve(listener: TcpListener) {
    for body in [
        r#"{"access_token":"new-access","refresh_token":"rotated-refresh","token_type":"Bearer","expires_in":3600}"#,
        r#"{"access_token":"newer-access","token_type":"Bearer","expires_in":3600}"#,
        r#"{"access_token":"bad-access","token_type":"Bearer","expires_in":3600,"scope":"mcp:read mcp:write"}"#,
    ] {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = read_request(&mut stream).await;
        assert!(request.contains("grant_type=refresh_token"));
        assert!(request.contains("resource=https%3A%2F%2Fapi.monarch.com%2Fmcp"));
        let reply = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(reply.as_bytes()).await.unwrap();
    }
}
