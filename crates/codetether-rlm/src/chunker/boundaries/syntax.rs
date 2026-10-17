//! Detect code, error, and heading boundaries.

use crate::chunker::types::ChunkType;
use std::collections::HashMap;

pub(super) fn classify_syntax_boundary(t: &str, i: usize, b: &mut HashMap<usize, (ChunkType, u8)>) {
    if t.starts_with("```") {
        b.insert(i, (ChunkType::Code, 4));
        return;
    }
    if t.starts_with('/') || t.starts_with("./") || t.starts_with("~/") {
        b.insert(i, (ChunkType::Code, 4));
        return;
    }
    let defs = [
        "function",
        "class ",
        "def ",
        "async function",
        "export",
        "fn ",
        "impl ",
        "struct ",
        "enum ",
    ];
    if defs.iter().any(|p| t.starts_with(p)) {
        b.insert(i, (ChunkType::Code, 5));
        return;
    }
    let low = t.to_lowercase();
    if low.starts_with("error")
        || low.contains("error:")
        || t.starts_with("Exception")
        || t.contains("FAILED")
    {
        b.insert(i, (ChunkType::Text, 8));
        return;
    }
    if t.starts_with('#') && t.len() > 2 && t.chars().nth(1) == Some(' ') {
        b.insert(i, (ChunkType::Text, 6));
    }
}
