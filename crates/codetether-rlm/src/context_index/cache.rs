//! Disk cache for context indexes.

use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use super::ContextIndex;

pub(super) fn hash(source: &str, content: &str) -> u64 {
    let mut h = DefaultHasher::new();
    "codetether-rlm-context-index-v1".hash(&mut h);
    source.hash(&mut h);
    content.hash(&mut h);
    h.finish()
}

pub(super) fn load(source: &str, content: &str) -> Option<ContextIndex> {
    let hash = hash(source, content);
    let raw = fs::read_to_string(path(hash)).ok()?;
    let index: ContextIndex = serde_json::from_str(&raw).ok()?;
    (index.hash == hash && index.source == source).then_some(index)
}

pub(super) fn store(index: &ContextIndex) {
    let dir = dir();
    if fs::create_dir_all(&dir).is_err() {
        return;
    }
    if let Ok(raw) = serde_json::to_string(index) {
        let _ = fs::write(dir.join(format!("{:016x}.json", index.hash)), raw);
    }
}

fn path(hash: u64) -> PathBuf {
    dir().join(format!("{hash:016x}.json"))
}

fn dir() -> PathBuf {
    std::env::temp_dir()
        .join("codetether-rlm")
        .join("context-index")
}
