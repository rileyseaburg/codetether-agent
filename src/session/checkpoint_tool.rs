//! Tool-result scanning for checkpoint state extraction.

use super::{checkpoint_text, checkpoint_walk::ScanState};
use crate::provider::ContentPart;

pub(super) fn scan(parts: &[ContentPart], state: &mut ScanState) {
    for part in parts {
        if let ContentPart::ToolResult { content, .. } = part {
            checkpoint_text::extract_browser_url(content, &mut state.browser_url);
            if content.contains("error") || content.contains("Error") {
                state
                    .blockers
                    .push(checkpoint_text::truncate_to_line(content, 200));
            }
        }
    }
}
