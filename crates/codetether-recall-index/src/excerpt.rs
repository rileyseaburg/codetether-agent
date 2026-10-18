//! Bounded transcript rendering for recall documents.

use codetether_provider_core::Message;

const MAX_EXCERPT_BYTES: usize = 4 * 1024;

pub(super) fn render(messages: &[Message], start: usize) -> String {
    let mut output = String::new();
    for (offset, message) in messages.iter().enumerate() {
        let role = codetether_context_policy::transcript::role_label(message.role);
        output.push_str(&format!("[{} {role}]\n", start + offset));
        for part in &message.content {
            output.push_str(&crate::part::render_part(part));
        }
        output.push_str("---\n");
        if output.len() >= MAX_EXCERPT_BYTES {
            break;
        }
    }
    truncate(output)
}

fn truncate(mut output: String) -> String {
    if output.len() <= MAX_EXCERPT_BYTES {
        return output;
    }
    let mut end = MAX_EXCERPT_BYTES;
    while !output.is_char_boundary(end) {
        end -= 1;
    }
    output.truncate(end);
    output.push_str("\n[...truncated]");
    output
}
