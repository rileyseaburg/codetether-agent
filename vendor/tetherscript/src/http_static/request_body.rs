//! Request body draining for static HTTP keep-alive parsing.

use std::io::{BufReader, Read};
use std::net::TcpStream;

pub(crate) const MAX_BODY_BYTES: usize = 1024 * 1024;

/// Consume a fixed-size request body before serving a static response.
pub(crate) fn drain_body(reader: &mut BufReader<TcpStream>, len: usize) -> Result<(), String> {
    if len > MAX_BODY_BYTES {
        return Err(format!("content-length {} exceeds {}", len, MAX_BODY_BYTES));
    }
    let mut body = vec![0; len];
    reader
        .read_exact(&mut body)
        .map_err(|e| format!("read body: {e}"))
}
