use super::super::intent::ApprovalIntent;
use super::{Action, action_from_rest};

pub(super) fn parse(prompt: &str) -> Option<Action<'_>> {
    let rest = crate::tui::app::text::command_with_optional_args(prompt, "/approve")?;
    let mut parts = rest.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or("");
    let more = parts.next().unwrap_or("").trim();
    let (intent, rest) = match first {
        "once" => (ApprovalIntent::ApproveOnce, more),
        "session" | "for-session" => (ApprovalIntent::ApproveForSession, more),
        _ if matches!(rest, "once") => (ApprovalIntent::ApproveOnce, ""),
        _ if matches!(rest, "session" | "for-session") => (ApprovalIntent::ApproveForSession, ""),
        _ => (ApprovalIntent::ApproveOnce, rest),
    };
    Some(action_from_rest(rest, intent))
}
