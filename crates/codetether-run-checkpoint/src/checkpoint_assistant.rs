//! Assistant-message scanning for checkpoint state extraction.

use super::{checkpoint_text, checkpoint_walk::ScanState};
use codetether_provider_core::ContentPart;

pub(super) fn scan(parts: &[ContentPart], state: &mut ScanState) {
    for part in parts {
        match part {
            ContentPart::ToolCall { id, name, .. } => state.actions.push(crate::ActionEvidence {
                call_id: id.clone(),
                tool: name.clone(),
                source_turn: state.source_turn,
                status: crate::ActionStatus::Attempted,
                result_turn: None,
            }),
            ContentPart::Text { text } if !text.is_empty() => {
                state.next_action = Some(checkpoint_text::truncate_to_line(text, 500));
                state.assistant_context = Some(text.clone());
            }
            ContentPart::Text { .. }
            | ContentPart::ToolResult { .. }
            | ContentPart::Image { .. }
            | ContentPart::File { .. }
            | ContentPart::Thinking { .. } => {}
        }
    }
}
