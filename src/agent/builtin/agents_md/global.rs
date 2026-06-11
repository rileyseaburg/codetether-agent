//! Codex-compatible global instruction discovery.

use std::path::PathBuf;

pub(super) fn instruction_file(max_bytes: usize) -> Option<(String, PathBuf)> {
    instruction_file_in(&codex_home()?, max_bytes)
}

pub(super) fn instruction_file_in(
    home: &std::path::Path,
    max_bytes: usize,
) -> Option<(String, PathBuf)> {
    let path = home.join("AGENTS.md");
    super::read::read_instruction_prefix(&path, max_bytes)
}

fn codex_home() -> Option<PathBuf> {
    std::env::var_os("CODEX_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".codex")))
}
