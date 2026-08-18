//! Summarize recent events when no model-generated context is available.

use super::ThoughtEvent;
use super::context_signals::collect;

/// Build a one-line summary of the most recent meaningful events.
pub(super) fn summarize(context: &[ThoughtEvent]) -> String {
    if context.is_empty() {
        return "No prior events recorded yet.".to_string();
    }
    let signals = collect(context);

    let mut lines = vec![format!(
        "{} recent cognition events are available.",
        context.len()
    )];
    if let Some(error) = signals.error {
        lines.push(format!("Latest error signal: {error}."));
    }
    if let Some(proposal) = signals.proposal {
        lines.push(format!("Recent proposal: {proposal}."));
    }
    if let Some(check) = signals.check {
        lines.push(format!("Recent check: {check}."));
    }
    lines.join(" ")
}
