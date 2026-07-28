//! HTTP transport streams for plain TCP and platform TLS.

use std::io::{Read, Write};
use std::net::TcpStream;
use std::time::Duration;

use crate::tls::{TlsConnector, TlsStream};

use super::http_url::ParsedHttpUrl;

pub(crate) trait HttpStream: Read + Write {}

impl HttpStream for TcpStream {}
impl HttpStream for TlsStream {}

pub(crate) fn connect(
    url: &ParsedHttpUrl,
    timeout: Duration,
) -> Result<Box<dyn HttpStream>, String> {
    if url.https {
        let connector =
            TlsConnector::new().map_err(|e| format!("http_request: create TLS connector: {e}"))?;
        let stream = connector.connect(&url.host, url.port).map_err(|e| {
            format!(
                "http_request: TLS handshake with {}:{} failed: {}",
                url.host, url.port, e
            )
        })?;
        return Ok(Box::new(stream));
    }
    let tcp = TcpStream::connect((url.host.as_str(), url.port)).map_err(|e| {
        format!(
            "http_request: connect to {}:{} failed: {}",
            url.host, url.port, e
        )
    })?;
    tcp.set_read_timeout(Some(timeout))
        .map_err(|e| format!("http_request: set read timeout failed: {e}"))?;
    tcp.set_write_timeout(Some(timeout))
        .map_err(|e| format!("http_request: set write timeout failed: {e}"))?;
    Ok(Box::new(tcp))
}
