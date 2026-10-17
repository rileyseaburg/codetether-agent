//! Detect conversational and tool-output chunk boundaries.

use super::types::ChunkType;
use std::collections::HashMap;
mod syntax;
use syntax::classify_syntax_boundary;

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
            let pri = if t.contains("FAILED") || t.contains("error") {
                7
            } else {
                3
            };
            boundaries.insert(i, (ChunkType::ToolOutput, pri));
            continue;
        }
        classify_syntax_boundary(t, i, &mut boundaries);
    }
    boundaries
}
