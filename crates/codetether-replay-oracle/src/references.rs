//! Resolve tool and file references to earlier turns.
use crate::tokens::extract_text_file_tokens;
use codetether_provider_core::{ContentPart, Message};
use std::collections::HashMap;

/// Compute which prior turns the `future` message references.
pub(crate) fn references_for_turn(
    future: &Message,
    current_idx: usize,
    files_per_turn: &[Vec<String>],
    tool_call_owner: &HashMap<String, usize>,
) -> Vec<usize> {
    let mut out = Vec::new();
    for part in &future.content {
        match part {
            ContentPart::ToolResult { tool_call_id, .. } => {
                if let Some(&owner) = tool_call_owner.get(tool_call_id) {
                    out.push(owner);
                }
            }
            ContentPart::Text { text } => {
                let file_refs = extract_text_file_tokens(text);
                for file in file_refs {
                    for (u, files) in files_per_turn.iter().enumerate().take(current_idx) {
                        if files.iter().any(|f| f == &file) {
                            out.push(u);
                            break;
                        }
                    }
                }
            }
            _ => {}
        }
    }
    out
}
