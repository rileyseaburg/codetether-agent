//! Extraction of notable signals from recent events.

use super::{ThoughtEvent, ThoughtEventType, placeholder, text_util};

/// Latest notable signals extracted from recent events.
#[derive(Default)]
pub(super) struct Signals {
    pub error: Option<String>,
    pub proposal: Option<String>,
    pub check: Option<String>,
}

impl Signals {
    fn complete(&self) -> bool {
        self.error.is_some() && self.proposal.is_some() && self.check.is_some()
    }
}

/// Scan newest-first, keeping the first usable value of each signal.
pub(super) fn collect(context: &[ThoughtEvent]) -> Signals {
    let mut signals = Signals::default();
    for event in context.iter().rev() {
        if signals.error.is_none() {
            signals.error = raw_field(event, "error", 140);
        }
        if signals.proposal.is_none() && event.event_type == ThoughtEventType::ProposalCreated {
            signals.proposal = clean_field(event, "title", 120);
        }
        if signals.check.is_none() && event.event_type == ThoughtEventType::CheckResult {
            signals.check = clean_field(event, "result_excerpt", 140);
        }
        if signals.complete() {
            break;
        }
    }
    signals
}

/// Read a non-empty string field from an event payload.
fn raw_field(event: &ThoughtEvent, key: &str, max_chars: usize) -> Option<String> {
    event
        .payload
        .get(key)
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(|value| text_util::trim_for_storage(value, max_chars))
}

/// Read a non-empty, placeholder-free string field from an event payload.
fn clean_field(event: &ThoughtEvent, key: &str, max_chars: usize) -> Option<String> {
    raw_field(event, key, max_chars)
        .filter(|value| !placeholder::has_template_placeholder_values(value))
}
