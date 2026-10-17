//! Unified diff parser for patch hunks.

use super::types::PatchHunk;

#[path = "parser_state.rs"]
mod state;

/// Parse all valid hunks from a unified diff patch.
///
/// # Arguments
/// * `patch` — Unified-diff text containing file headers and counted hunks.
/// # Returns
/// Hunks in encounter order; malformed headers are ignored by the parser.
/// # Examples
/// ```
/// let hunks = codetether_tool_core::patch::parse_patch("+++ b/note.txt\n@@ -1 +1 @@\n-old\n+new");
/// assert_eq!(hunks[0].file, "note.txt");
/// ```
pub fn parse_patch(patch: &str) -> Vec<PatchHunk> {
    let mut parser = state::PatchParser::default();
    for line in patch.lines() {
        parser.absorb(line);
    }
    parser.finish()
}
