//! Parse static HTTP request metadata from a request-head buffer.

use super::request_header_scan::apply_header;

/// Borrowed request metadata parsed from the request head.
pub(crate) struct ParsedHead<'a> {
    pub method: &'a str,
    pub target: &'a str,
    pub version: &'a str,
    pub close: bool,
    pub keep_alive: bool,
    pub content_length: usize,
}

/// Parse start-line and relevant headers from a request-head buffer.
pub(crate) fn parse_head(head: &[u8]) -> Result<ParsedHead<'_>, String> {
    let text = std::str::from_utf8(head).map_err(|e| format!("request head utf-8: {e}"))?;
    let mut lines = text.lines();
    let (method, target, version) = parse_start(lines.next().ok_or("missing start-line")?)?;
    let mut parsed = ParsedHead {
        method,
        target,
        version,
        close: false,
        keep_alive: false,
        content_length: 0,
    };
    for line in lines {
        let line = line.trim_end_matches('\r');
        if line.is_empty() {
            break;
        }
        apply_header(line, &mut parsed);
    }
    Ok(parsed)
}

fn parse_start(line: &str) -> Result<(&str, &str, &str), String> {
    let mut parts = line.splitn(3, ' ');
    let method = parts.next().ok_or("missing method")?;
    let target = parts.next().ok_or("missing target")?;
    let version = parts.next().unwrap_or("HTTP/1.1");
    Ok((method, target, version))
}
