//! Preserve interactive corrections as pinned human input, beside the tool result.
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;
use codetether_context_policy::pages::PageKind;

pub(super) fn append(session: &mut Session, call_id: &str, feedback: String) {
    session.add_human_message(Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: format!("Feedback on denied tool call {call_id}:\n{feedback}"),
        }],
    });
    if let Some(page) = session.pages.last_mut() {
        *page = PageKind::Constraint;
    }
}
#[cfg(test)]
#[path = "feedback_tests.rs"]
mod tests;
