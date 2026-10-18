//! Sidecar attachment and composer placeholder insertion.
use super::Host;
pub(super) fn attach_summarised_paste(host: &mut impl Host, normalized: &str) {
    let line_count = normalized.lines().count();
    let size = crate::format_size(normalized.len());
    let placeholder = crate::attach_paste(host.pending_pastes(), normalized.to_string());
    let id = host.pending_pastes().last().map(|p| p.id).unwrap_or(0);
    host.set_command_mode(if host.input().starts_with('/') {
        true
    } else {
        false
    });
    host.insert_text(&placeholder);
    host.set_status(format!(
        "Pasted {line_count} lines ({size}) summarised as #{id}; full text will be sent to the agent."
    ));
}
