//! HTTP response parsing for the browser bridge transport.

use std::io::BufRead;

pub(crate) fn read_status<R: BufRead>(reader: &mut R) -> Result<u16, String> {
    let mut status_line = String::new();
    reader
        .read_line(&mut status_line)
        .map_err(|e| format!("browser bridge: read status failed: {}", e))?;
    status_line
        .split_whitespace()
        .nth(1)
        .ok_or_else(|| "browser bridge: missing HTTP status".to_string())?
        .parse()
        .map_err(|_| {
            format!(
                "browser bridge: bad HTTP status line {}",
                status_line.trim()
            )
        })
}

pub(crate) fn read_headers<R: BufRead>(reader: &mut R) -> Result<Option<usize>, String> {
    let mut content_length = None;
    loop {
        let mut line = String::new();
        reader
            .read_line(&mut line)
            .map_err(|e| format!("browser bridge: read header failed: {}", e))?;
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if trimmed.is_empty() {
            break;
        }
        if let Some((name, value)) = trimmed.split_once(':') {
            if name.eq_ignore_ascii_case("content-length") {
                content_length = value.trim().parse().ok();
            }
        }
    }
    Ok(content_length)
}
