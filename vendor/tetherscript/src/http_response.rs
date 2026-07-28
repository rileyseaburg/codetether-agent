//! Response serialization for `http_serve`.

use std::collections::HashMap;
use std::io::{self, Write};
use std::net::TcpStream;

use super::http_response_extract::extract;
use super::http_status::reason_phrase;
use crate::value::Value;

/// Write a TetherScript HTTP response value to the socket.
pub(crate) fn write_response(
    stream: &mut TcpStream,
    resp: &Value,
    keep_alive: bool,
) -> Result<(), String> {
    let (status, headers, body) = extract(resp)?;
    write_parts(
        stream,
        status,
        reason_phrase(status),
        headers,
        body,
        keep_alive,
    )
    .map_err(|e| format!("write response: {e}"))
}

/// Write a simple text/byte response without going through script values.
pub(crate) fn write_simple(
    stream: &mut TcpStream,
    status: u16,
    content_type: &str,
    body: &[u8],
    keep_alive: bool,
) -> Result<(), String> {
    let headers = HashMap::from([("content-type".to_string(), content_type.to_string())]);
    write_parts(
        stream,
        status,
        reason_phrase(status),
        headers,
        body.to_vec(),
        keep_alive,
    )
    .map_err(|e| e.to_string())
}

fn write_parts(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    headers: HashMap<String, String>,
    body: Vec<u8>,
    keep_alive: bool,
) -> io::Result<()> {
    let content_type = headers
        .get("content-type")
        .cloned()
        .unwrap_or_else(|| "text/plain; charset=utf-8".to_string());
    let connection = if keep_alive { "keep-alive" } else { "close" };
    let mut head = format!("HTTP/1.1 {status} {reason}\r\nContent-Length: {}\r\nContent-Type: {content_type}\r\nConnection: {connection}\r\n", body.len());
    for (k, v) in &headers {
        append_header(&mut head, k, v);
    }
    head.push_str("\r\n");
    stream.write_all(head.as_bytes())?;
    stream.write_all(&body)?;
    stream.flush()
}

fn append_header(head: &mut String, k: &str, v: &str) {
    if k == "content-length" || k == "content-type" || k == "connection" {
        return;
    }
    head.push_str(k);
    head.push_str(": ");
    head.push_str(v);
    head.push_str("\r\n");
}
