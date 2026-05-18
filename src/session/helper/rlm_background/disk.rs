//! Std filesystem cache for completed background RLM summaries.

use std::fs;
use std::path::PathBuf;

pub(super) fn read(key: u64) -> Option<String> {
    fs::read_to_string(path(key)).ok()
}

pub(super) fn write(key: u64, summary: &str) {
    let dir = dir();
    if fs::create_dir_all(&dir).is_ok() {
        let _ = fs::write(dir.join(format!("{key:016x}.txt")), summary);
    }
}

fn path(key: u64) -> PathBuf {
    dir().join(format!("{key:016x}.txt"))
}

fn dir() -> PathBuf {
    std::env::var("CODETETHER_RLM_BG_CACHE_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::temp_dir()
                .join("codetether-rlm")
                .join("background")
        })
}
