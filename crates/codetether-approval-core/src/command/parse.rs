use super::intent::ApprovalIntent;
#[path = "approve.rs"]
mod approve;

const COMMANDS: &[(&str, ApprovalIntent)] = &[
    ("/approve-once", ApprovalIntent::ApproveOnce),
    ("/approve-session", ApprovalIntent::ApproveForSession),
    ("/approve-for-session", ApprovalIntent::ApproveForSession),
    ("/deny", ApprovalIntent::Deny),
    ("/abort", ApprovalIntent::Abort),
];

/// Parsed approval intent, optional request ID and feedback reason.
///
/// # Examples
/// ```
/// let action = codetether_approval_core::command::Action::parse("/approve once request-1").unwrap();
/// assert_eq!(action.id, Some("request-1"));
/// ```
pub struct Action<'a> {
    /// Decision selected by the command.
    pub intent: ApprovalIntent,
    /// Explicit ID, or None when the host should select a pending request.
    pub id: Option<&'a str>,
    /// Explicit feedback or the intent's default reason.
    pub reason: String,
}

impl<'a> Action<'a> {
    /// Parse a supported command without applying a decision.
    ///
    /// # Arguments
    /// * `prompt` — Complete command text.
    ///
    /// # Returns
    /// A parsed action, or None when the command is not recognized.
    ///
    /// # Examples
    /// ```
    /// assert!(codetether_approval_core::command::Action::parse("plain text").is_none());
    /// ```
    pub fn parse(prompt: &'a str) -> Option<Self> {
        COMMANDS
            .iter()
            .find_map(|(command, intent)| parse(prompt, command, *intent))
            .or_else(|| approve::parse(prompt))
    }
}

fn parse<'a>(prompt: &'a str, command: &str, intent: ApprovalIntent) -> Option<Action<'a>> {
    let rest = codetether_slash_command::command_with_optional_args(prompt, command)?;
    Some(from_rest(rest, intent))
}

fn from_rest<'a>(rest: &'a str, intent: ApprovalIntent) -> Action<'a> {
    let mut parts = rest.splitn(2, char::is_whitespace);
    let id = parts.next().map(str::trim).filter(|id| !id.is_empty());
    let reason = parts
        .next()
        .map(str::trim)
        .filter(|reason| !reason.is_empty())
        .unwrap_or(intent.fallback_reason())
        .to_string();
    Action { intent, id, reason }
}

pub(super) fn action_from_rest<'a>(rest: &'a str, intent: ApprovalIntent) -> Action<'a> {
    from_rest(rest, intent)
}
