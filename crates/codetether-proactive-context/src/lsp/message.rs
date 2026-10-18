//! Format prefetched LSP sections into a system message.
use codetether_provider_core::Message;
pub(super) fn from_sections(sections: Vec<String>) -> Option<Message> {
    if sections.is_empty() {
        return None;
    }

    Some(Message {
        role: codetether_provider_core::Role::System,
        content: vec![codetether_provider_core::ContentPart::Text {
            text: format!(
                "Mandatory proactive LSP context (prefetched before first reply). Prioritize these real LSP diagnostics and errors over speculation. Do not call the lsp tool just to rediscover the same issues unless you need deeper navigation detail.\n\n{}",
                sections.join("\n\n---\n\n")
            ),
        }],
    })
}
