//! Single-cap shrinking pass over retained messages.

use crate::provider::{ContentPart, Message, Role};

use super::shrink_part::shrink_content_part;

/// Shrink every payload above `cap_bytes`; the leading `[CONTEXT
/// TRUNCATED]` marker and (unless `include_latest_user`) the newest
/// user turn are protected. Returns the number of parts changed.
pub(super) fn shrink_payloads_over_cap(
    messages: &mut [Message],
    cap_bytes: usize,
    include_latest_user: bool,
) -> usize {
    let message_count = messages.len();
    let mut changed = 0;
    for (msg_idx, msg) in messages.iter_mut().enumerate() {
        let role = msg.role;
        let is_latest_user = matches!(role, Role::User) && msg_idx + 1 == message_count;
        for part in &mut msg.content {
            let cap = match part {
                ContentPart::ToolResult { .. } | ContentPart::Thinking { .. } => cap_bytes,
                ContentPart::ToolCall { .. } => cap_bytes,
                ContentPart::Text { text } => {
                    if msg_idx == 0 && text.starts_with("[CONTEXT TRUNCATED]") {
                        continue;
                    }
                    if is_latest_user && !include_latest_user {
                        cap_bytes.max(16_384)
                    } else {
                        cap_bytes
                    }
                }
                ContentPart::Image { .. } | ContentPart::File { .. } => continue,
            };
            if shrink_content_part(part, cap) {
                changed += 1;
            }
        }
    }
    changed
}
