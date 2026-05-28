//! Assistant-message scanning for checkpoint state extraction.

use super::{checkpoint_text, checkpoint_walk::ScanState};
use crate::provider::ContentPart;

pub(super) fn scan(parts: &[ContentPart], state: &mut ScanState) {
    for part in parts {
        match part {
            ContentPart::ToolCall { name, .. } => state.completed_actions.push(name.clone()),
            ContentPart::Text { text } if !text.is_empty() => {
                state.next_action = Some(checkpoint_text::truncate_to_line(text, 500));
            }
            ContentPart::Text { .. }
            | ContentPart::ToolResult { .. }
            | ContentPart::Image { .. }
            | ContentPart::File { .. }
            | ContentPart::Thinking { .. } => {}
        }
    }
}