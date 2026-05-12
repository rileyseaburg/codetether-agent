//! Semantic boundary detection for chunk splitting.

use std::collections::HashMap;

use super::types::ChunkType;

/// Find semantic boundaries in `lines`.
pub fn find_boundaries(lines: &[&str]) -> HashMap<usize, (ChunkType, u8)> {
    let mut boundaries = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim();
        if t.starts_with("[User]:") || t.starts_with("[Assistant]:") {
            boundaries.insert(i, (ChunkType::Conversation, 5));
            continue;
        }
        if t.starts_with("[Tool ") {
            let pri = if t.contains("FAILED") || t.contains("error") { 7 } else { 3 };
            boundaries.insert(i, (ChunkType::ToolOutput, pri));
            continue;
        }
        classify_syntax_boundary(t, i, &mut boundaries);
    }
    boundaries
}

fn classify_syntax_boundary(
    t: &str, i: usize, b: &mut HashMap<usize, (ChunkType, u8)>,
) {
    if t.starts_with("```") { b.insert(i, (ChunkType::Code, 4)); return; }
    if t.starts_with('/') || t.starts_with("./") || t.starts_with("~/") { b.insert(i, (ChunkType::Code, 4)); return; }
    let defs = ["function", "class ", "def ", "async function", "export", "fn ", "impl ", "struct ", "enum "];
    if defs.iter().any(|p| t.starts_with(p)) { b.insert(i, (ChunkType::Code, 5)); return; }
    let low = t.to_lowercase();
    if low.starts_with("error") || low.contains("error:") || t.starts_with("Exception") || t.contains("FAILED") {
        b.insert(i, (ChunkType::Text, 8)); return;
    }
    if t.starts_with('#') && t.len() > 2 && t.chars().nth(1) == Some(' ') { b.insert(i, (ChunkType::Text, 6)); }
}