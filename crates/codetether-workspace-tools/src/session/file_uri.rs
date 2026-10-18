//! File-URI construction for workspace-scoped LSP tools.

use std::path::Path;

/// Build a percent-encoded file URI for an LSP workspace root.
pub(super) fn build(path: &Path) -> String {
    let path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
    let raw = path.to_string_lossy().replace('\\', "/");
    format!(
        "file://{}",
        raw.bytes().map(encode_byte).collect::<String>()
    )
}

fn encode_byte(byte: u8) -> String {
    match byte {
        b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'/' | b'-' | b'_' | b'.' | b'~' => {
            (byte as char).to_string()
        }
        _ => format!("%{byte:02X}"),
    }
}
