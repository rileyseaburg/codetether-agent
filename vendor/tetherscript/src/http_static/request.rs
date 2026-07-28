//! HTTP request parsing for the static fast path.

use std::io::BufReader;
use std::net::TcpStream;

use super::request_body::drain_body;
use super::request_head::read_head;
use super::request_parse::parse_head;

/// Minimal request data needed to serve cached static responses.
pub(crate) struct StaticRequest {
    pub method: String,
    pub path: String,
    pub keep_alive: bool,
}

/// Read one request from a keep-alive connection.
pub(crate) fn read(reader: &mut BufReader<TcpStream>) -> Result<Option<StaticRequest>, String> {
    let Some(head) = read_head(reader)? else {
        return Ok(None);
    };
    let parsed = parse_head(&head)?;
    drain_body(reader, parsed.content_length)?;
    let path = parsed
        .target
        .split_once('?')
        .map_or(parsed.target, |(path, _)| path);
    Ok(Some(StaticRequest {
        method: parsed.method.to_string(),
        path: if path.is_empty() {
            "/".into()
        } else {
            path.into()
        },
        keep_alive: (parsed.version != "HTTP/1.0" && !parsed.close) || parsed.keep_alive,
    }))
}
