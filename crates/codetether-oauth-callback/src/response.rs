//! OAuth callback response handling.
use anyhow::{Context, Result};
use tokio::io::AsyncWriteExt;

pub(super) async fn write_http_response(
    stream: &mut tokio::net::TcpStream,
    status_code: u16,
    status_text: &str,
    body: &str,
) -> Result<()> {
    let response = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        status_code,
        status_text,
        body.len(),
        body
    );
    stream
        .write_all(response.as_bytes())
        .await
        .context("Failed to write callback response")?;
    Ok(())
}

pub(super) async fn accepted(stream: &mut tokio::net::TcpStream) -> Result<()> {
    write_http_response(stream, 200, "OK", "<html><body><h1>CodeTether login complete</h1><p>You can close this tab.</p></body></html>").await
}

pub(super) async fn rejected(stream: &mut tokio::net::TcpStream) -> Result<()> {
    write_http_response(stream, 400, "Bad Request", "<html><body><h1>Invalid callback request</h1><p>Retry authorization from CodeTether.</p></body></html>").await
}
