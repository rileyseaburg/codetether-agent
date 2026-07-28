//! HTTP request writing for the browser bridge transport.

use std::io::Write;
use std::net::TcpStream;

use super::super::bridge_url::BridgeUrl;

pub(crate) fn write_request(
    stream: &mut TcpStream,
    parsed: &BridgeUrl,
    body: &str,
    user_agent: &str,
) -> Result<(), String> {
    write!(stream, "POST {} HTTP/1.1\r\nHost: {}\r\nUser-Agent: {}\r\nContent-Type: application/json\r\nAccept: application/json\r\nConnection: close\r\nContent-Length: {}\r\n\r\n{}", parsed.target, parsed.host_header, user_agent, body.len(), body)
        .and_then(|_| stream.flush())
        .map_err(|e| format!("browser bridge: write request failed: {}", e))
}
