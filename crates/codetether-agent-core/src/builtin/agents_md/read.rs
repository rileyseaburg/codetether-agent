//! Capped instruction-file reads for AGENTS.md discovery.

use std::io::Read;
use std::path::{Path, PathBuf};

const INSTRUCTION_FILES: [&str; 2] = ["AGENTS.override.md", "AGENTS.md"];

pub(super) fn preferred_instruction_file(
    dir: &Path,
    max_bytes: usize,
) -> Option<(String, PathBuf)> {
    INSTRUCTION_FILES
        .iter()
        .filter_map(|name| read_instruction_prefix(&dir.join(name), max_bytes))
        .next()
}

pub(super) fn read_instruction_prefix(path: &Path, max_bytes: usize) -> Option<(String, PathBuf)> {
    if max_bytes == 0 || !path.is_file() {
        return None;
    }
    let mut file = std::fs::File::open(path).ok()?;
    let mut bytes = Vec::new();
    file.by_ref()
        .take(max_bytes as u64)
        .read_to_end(&mut bytes)
        .ok()?;
    let content = valid_utf8_prefix(&bytes).to_string();
    if content.trim().is_empty() {
        return None;
    }
    Some((content, path.to_path_buf()))
}

fn valid_utf8_prefix(bytes: &[u8]) -> &str {
    match std::str::from_utf8(bytes) {
        Ok(content) => content,
        Err(err) => std::str::from_utf8(&bytes[..err.valid_up_to()]).unwrap_or(""),
    }
}
