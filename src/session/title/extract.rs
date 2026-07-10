//! Pure helpers for deriving a title string from session messages.

use super::super::helper::text::truncate_with_ellipsis;
use super::super::types::Session;

/// Derive a truncated title from the first user message, if any.
///
/// Returns `None` when the session has no user message yet.
pub(super) fn title_from_first_user_message(session: &Session) -> Option<String> {
    session.messages.iter().find_map(|message| {
        if message.role != crate::provider::Role::User {
            return None;
        }
        let text = message
            .content
            .iter()
            .filter_map(|part| match part {
                crate::provider::ContentPart::Text { text } => Some(text.as_str()),
                _ => None,
            })
            .collect::<Vec<_>>()
            .join(" ");
        is_title_candidate(&text).then(|| truncate_with_ellipsis(&text, 47))
    })
}

pub(crate) fn is_title_candidate(text: &str) -> bool {
    let text = text.trim_start();
    !text.is_empty()
        && !text.starts_with("# AGENTS.md instructions for ")
        && !text.starts_with("<environment_context>")
}

#[cfg(test)]
mod tests {
    use super::is_title_candidate;

    #[test]
    fn rejects_injected_codex_context() {
        assert!(!is_title_candidate(
            "# AGENTS.md instructions for /tmp/x\n<INSTRUCTIONS>"
        ));
        assert!(!is_title_candidate(
            "<environment_context>\n<cwd>/tmp/x</cwd>"
        ));
        assert!(is_title_candidate("see the elevon door hanger work"));
    }
}
