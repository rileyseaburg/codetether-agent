use super::intent::ApprovalIntent;
#[path = "approval_command_parse_approve.rs"]
mod approve;

const COMMANDS: &[(&str, ApprovalIntent)] = &[
    ("/approve-once", ApprovalIntent::ApproveOnce),
    ("/approve-session", ApprovalIntent::ApproveForSession),
    ("/approve-for-session", ApprovalIntent::ApproveForSession),
    ("/deny", ApprovalIntent::Deny),
    ("/abort", ApprovalIntent::Abort),
];

pub(super) struct Action<'a> {
    pub intent: ApprovalIntent,
    pub id: Option<&'a str>,
    pub reason: String,
}

impl<'a> Action<'a> {
    pub fn parse(prompt: &'a str) -> Option<Self> {
        COMMANDS
            .iter()
            .find_map(|(command, intent)| parse(prompt, command, *intent))
            .or_else(|| approve::parse(prompt))
    }
}

fn parse<'a>(prompt: &'a str, command: &str, intent: ApprovalIntent) -> Option<Action<'a>> {
    let rest = crate::tui::app::text::command_with_optional_args(prompt, command)?;
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
