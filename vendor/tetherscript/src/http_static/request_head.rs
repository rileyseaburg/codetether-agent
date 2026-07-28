//! Bounded request-head reader for static HTTP parsing.

use std::io::{BufRead, BufReader, ErrorKind};
use std::net::TcpStream;

const MAX_HEAD_BYTES: usize = 64 * 1024;

/// Read bytes through the header terminator without consuming following bytes.
pub(crate) fn read_head(reader: &mut BufReader<TcpStream>) -> Result<Option<Vec<u8>>, String> {
    let mut head = Vec::new();
    loop {
        let available = match reader.fill_buf() {
            Ok(bytes) => bytes,
            Err(e) if idle(&e) && head.is_empty() => return Ok(None),
            Err(e) => return Err(format!("read request head: {e}")),
        };
        if available.is_empty() {
            return if head.is_empty() {
                Ok(None)
            } else {
                Err("incomplete request head".into())
            };
        }
        let before = head.len();
        head.extend_from_slice(available);
        if head.len() > MAX_HEAD_BYTES {
            return Err(format!("request head exceeds {MAX_HEAD_BYTES} bytes"));
        }
        if let Some(end) = header_end(&head) {
            let consume_len = end - before;
            reader.consume(consume_len);
            head.truncate(end);
            return Ok(Some(head));
        }
        let consume_len = available.len();
        reader.consume(consume_len);
    }
}

fn header_end(bytes: &[u8]) -> Option<usize> {
    bytes
        .windows(4)
        .position(|w| w == b"\r\n\r\n")
        .map(|pos| pos + 4)
        .or_else(|| {
            bytes
                .windows(2)
                .position(|w| w == b"\n\n")
                .map(|pos| pos + 2)
        })
}

fn idle(error: &std::io::Error) -> bool {
    matches!(error.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut)
}
