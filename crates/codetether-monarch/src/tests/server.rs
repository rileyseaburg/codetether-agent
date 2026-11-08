//! Local lifecycle server with deliberately fragmented SSE writes.
use super::{replies::for_step, request::read_request};
use tokio::{io::AsyncWriteExt, net::TcpListener};

pub(super) async fn serve(listener: TcpListener) {
    for step in 0..4 {
        let (mut stream, _) = listener.accept().await.unwrap();
        let (headers, request) = read_request(&mut stream).await;
        assert!(headers.contains("authorization: bearer fixture-only"));
        if step > 0 {
            assert!(headers.contains("mcp-session-id: local-session"));
        }
        let (status, content_type, body) = for_step(step, &headers, &request);
        let reply = format!(
            "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nMcp-Session-Id: local-session\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        for part in reply.as_bytes().chunks(17) {
            stream.write_all(part).await.unwrap();
        }
    }
}
