//! Message serialization for context summarization.

use super::error_context_part::write_part;
use super::text::role_label;
use crate::provider::Message;

/// Render provider messages as plain text for RLM summarization.
pub fn messages_to_rlm_context(messages: &[Message]) -> String {
    let mut out = String::new();
    for (idx, m) in messages.iter().enumerate() {
        out.push_str(&format!("[{} {}]\n", idx, role_label(m.role)));
        m.content.iter().for_each(|part| write_part(&mut out, part));
        out.push_str("\n---\n\n");
    }
    out
}
