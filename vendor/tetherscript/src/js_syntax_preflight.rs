#[path = "js_syntax_preflight/skip.rs"]
mod skip;

pub(crate) fn reject(source: &str) -> Result<(), String> {
    let bytes = source.as_bytes();
    let mut i = 0;
    let mut line = 1;
    let mut col = 1;
    while i < bytes.len() {
        let byte = bytes[i];
        if byte == b'\n' {
            line += 1;
            col = 0;
        }
        if byte == b'\'' || byte == b'"' {
            (i, line, col) = skip::string(bytes, i, line, col);
            continue;
        }
        if byte == b'`' {
            (i, line, col) = skip::template(bytes, i, line, col);
            continue;
        }
        if byte == b'/' && matches!(bytes.get(i + 1), Some(b'/' | b'*')) {
            (i, line, col) = skip::comment(bytes, i, line, col);
            continue;
        }
        i += 1;
        col += 1;
    }
    Ok(())
}
