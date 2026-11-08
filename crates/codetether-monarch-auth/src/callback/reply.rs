//! Bounded callback responses without credentials, caching or referrer leakage.
use super::Callback;
use anyhow::Result;
use std::time::Duration;
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub(super) async fn reply(stream: &mut TcpStream, parsed: &Result<Callback>) {
    let (status, body) = match parsed {
        Ok(Callback::Code(_)) => (
            "200 OK",
            "Authorization received. Return to CodeTether to finish connecting.",
        ),
        Ok(Callback::Denied) => (
            "400 Bad Request",
            "Authorization was denied. Return to CodeTether.",
        ),
        Err(_) => ("400 Bad Request", "Invalid authorization callback."),
    };
    let response = format!(
        "HTTP/1.1 {status}\r\nContent-Type: text/plain\r\nCache-Control: no-store\r\nReferrer-Policy: no-referrer\r\nContent-Security-Policy: default-src 'none'\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{body}",
        body.len()
    );
    let _ = tokio::time::timeout(
        Duration::from_secs(2),
        stream.write_all(response.as_bytes()),
    )
    .await;
}
