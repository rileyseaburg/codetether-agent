//! Text views of provider messages for context summarization and recall.
//!
//! [messages_to_rlm_context] renders an ordered transcript; [role_label],
//! [extract_text_content] and [latest_user_text] provide its text-selection helpers.
//! These functions perform no I/O and do not invoke a model.
use codetether_provider_core::Message;
mod part;
mod role;
mod text;
pub use role::role_label;
pub use text::{extract_text_content, latest_user_text};

/// Render ordered messages and their content blocks for RLM context.
///
/// # Arguments
/// * `messages` - Source messages, in conversation order.
///
/// # Returns
/// A transcript with zero-based indices, role labels and content markers.
///
/// # Examples
/// ```
/// use codetether_context_policy::transcript::messages_to_rlm_context;
/// use codetether_provider_core::{Message, Role};
/// let messages = [Message { role: Role::User, content: vec![] }];
/// assert_eq!(messages_to_rlm_context(&messages), "[0 User]\n\n---\n\n");
/// ```
pub fn messages_to_rlm_context(messages: &[Message]) -> String {
    let mut out = String::new();
    for (idx, m) in messages.iter().enumerate() {
        out.push_str(&format!("[{} {}]\n", idx, role_label(m.role)));

        for part in &m.content {
            part::append(&mut out, part);
        }

        out.push_str("\n---\n\n");
    }
    out
}
