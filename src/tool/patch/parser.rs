//! Unified diff parser for patch hunks.

use super::types::PatchHunk;

#[path = "parser_state.rs"]
mod state;

/// Parse all valid hunks from a unified diff patch.
pub(super) fn parse_patch(patch: &str) -> Vec<PatchHunk> {
    let mut parser = state::PatchParser::default();
    for line in patch.lines() {
        parser.absorb(line);
    }
    parser.finish()
}
