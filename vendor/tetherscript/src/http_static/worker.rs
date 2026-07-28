//! Per-connection keep-alive handling for cached static responses.

use std::io::{BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

use super::request;
use super::site::Site;

const KEEP_ALIVE_TIMEOUT: Duration = Duration::from_secs(5);

/// Serve all requests on one accepted connection.
pub(crate) fn handle(stream: TcpStream, site: &Site) -> Result<(), String> {
    stream
        .set_read_timeout(Some(KEEP_ALIVE_TIMEOUT))
        .map_err(|e| format!("set keep-alive timeout: {e}"))?;
    let mut reader = BufReader::new(stream);
    loop {
        let request = match request::read(&mut reader) {
            Ok(Some(request)) => request,
            Ok(None) => return Ok(()),
            Err(e) => {
                return write_and_close(reader.get_mut(), site.bad_request.bytes("GET", false), e)
            }
        };
        let response = response_for(site, &request);
        reader
            .get_mut()
            .write_all(response.bytes(&request.method, request.keep_alive))
            .map_err(|e| format!("write response: {e}"))?;
        if !request.keep_alive {
            return Ok(());
        }
    }
}

fn response_for<'a>(
    site: &'a Site,
    request: &request::StaticRequest,
) -> &'a super::cache::CachedResponse {
    if request.method != "GET" && request.method != "HEAD" {
        return &site.method_not_allowed;
    }
    site.route(&request.path).unwrap_or(&site.not_found)
}

fn write_and_close(stream: &mut TcpStream, response: &[u8], error: String) -> Result<(), String> {
    let _ = stream.write_all(response);
    Err(format!("parse: {}", error))
}
