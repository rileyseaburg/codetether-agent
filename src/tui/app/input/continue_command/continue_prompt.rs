//! Resolve the prompt to resume when the user runs `/continue`.
//!
//! When a socket silently drops, the in-flight prompt is the natural thing to
//! re-run. If nothing is in flight, use the last user message in the
//! transcript so an idle session can still be nudged forward. A literal
//! `"continue"` is the last resort when no prior user message exists.

use crate::provider::{ContentPart, Role};
use crate::session::Session;
use crate::tui::app::state::App;

/// Pick the best prompt to resubmit for `/continue`.
///
/// Prefers the in-flight prompt (the turn that stalled), then the most recent
/// user message in `session`, then a literal `"continue"` nudge.
pub(super) fn resolve(app: &App, session: Option<&Session>) -> String {
    if let Some(prompt) = app.state.main_inflight_prompt.clone() {
        if !prompt.trim().is_empty() {
            return prompt;
        }
    }
    if let Some(session) = session {
        if let Some(prompt) = crate::session::tasks::runtime::resume_prompt(&session.id) {
            return prompt;
        }
        if let Some(text) = last_user_text(session) {
            return text;
        }
    }
    "continue".to_string()
}

/// Extract the text of the most recent [`Role::User`] message.
fn last_user_text(session: &Session) -> Option<String> {
    session
        .messages
        .iter()
        .rev()
        .find(|m| m.role == Role::User)
        .and_then(|m| {
            m.content.iter().find_map(|part| match part {
                ContentPart::Text { text } if !text.trim().is_empty() => Some(text.clone()),
                _ => None,
            })
        })
}
