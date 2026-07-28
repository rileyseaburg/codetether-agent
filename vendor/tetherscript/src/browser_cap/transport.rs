//! Blocking HTTP transport for browserctl-compatible local bridges.

use std::io::{BufReader, Read};
use std::net::TcpStream;
use std::time::Duration;

use super::bridge_url::BridgeUrl;

#[path = "http_request.rs"]
mod http_request;
#[path = "http_response.rs"]
mod http_response;

use http_response::{read_headers, read_status};

const USER_AGENT: &str = "tetherscript-browser-cap/0.1";

pub(crate) fn post_json(url: &str, body: &str, timeout: Duration) -> Result<String, String> {
    let parsed = BridgeUrl::parse(url)?;
    let mut stream = connect(&parsed, timeout)?;
    http_request::write_request(&mut stream, &parsed, body, USER_AGENT)?;
    read_response(stream)
}

fn connect(parsed: &BridgeUrl, timeout: Duration) -> Result<TcpStream, String> {
    let stream = TcpStream::connect((parsed.host.as_str(), parsed.port)).map_err(|e| {
        format!(
            "browser bridge: connect to {}:{} failed: {}",
            parsed.host, parsed.port, e
        )
    })?;
    stream
        .set_read_timeout(Some(timeout))
        .map_err(|e| format!("browser bridge: set read timeout failed: {}", e))?;
    stream
        .set_write_timeout(Some(timeout))
        .map_err(|e| format!("browser bridge: set write timeout failed: {}", e))?;
    Ok(stream)
}

fn read_response(stream: TcpStream) -> Result<String, String> {
    let mut reader = BufReader::new(stream);
    let status = read_status(&mut reader)?;
    let content_length = read_headers(&mut reader)?;
    let mut bytes = Vec::new();
    match content_length {
        Some(n) => reader.take(n as u64).read_to_end(&mut bytes),
        None => reader.read_to_end(&mut bytes),
    }
    .map_err(|e| format!("browser bridge: read body failed: {}", e))?;
    let text = String::from_utf8_lossy(&bytes).into_owned();
    if (200..300).contains(&status) {
        Ok(text)
    } else {
        Err(format!("browser bridge returned HTTP {}: {}", status, text))
    }
}
