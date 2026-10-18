//! Build serializable scope evidence from a session identifier and messages.
use serde::{Deserialize, Serialize};

use codetether_provider_core::Message;

use super::scope_item::ScopeItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Persisted scope and evidence state for one session.
/// # Examples
/// ```
/// let ledger = codetether_evidence_ledger::ledger::build("s", &[]);
/// assert_eq!(ledger.session_id, "s");
/// ```
pub struct ScopeLedger {
    /// Session identifier used for ledger persistence.
    pub session_id: String,
    /// Requested deliverables and their evidence summaries.
    pub items: Vec<ScopeItem>,
    /// Extracted observations in their original category order.
    pub evidence: Vec<codetether_evidence_core::record::EvidenceRecord>,
}

/// Build scope items and evidence from the supplied session messages.
/// # Arguments
/// `session_id` identifies the session; `messages` supplies all current messages.
/// # Returns
/// A ledger using the latest user bullet list and all extracted evidence.
/// # Examples
/// ```rust
/// let ledger = codetether_evidence_ledger::ledger::build("s", &[]);
/// assert!(ledger.items.is_empty());
/// ```
pub fn build(session_id: &str, messages: &[Message]) -> ScopeLedger {
    let evidence =
        codetether_evidence_core::extract::from_text(&super::message_text::joined(messages));
    let status = status_for(&evidence);
    let items = super::scope_extract::from_messages(messages)
        .into_iter()
        .map(|item| ScopeItem::new(item, status.clone(), &evidence))
        .collect();
    ScopeLedger {
        session_id: session_id.to_string(),
        items,
        evidence,
    }
}

/// Count deliverables not named in the answer, ignoring ASCII case.
/// # Arguments
/// `ledger` supplies deliverables; `answer` is the candidate final response.
/// # Returns
/// The number of unmatched deliverable strings.
/// # Examples
/// ```rust
/// let ledger = codetether_evidence_ledger::ledger::build("s", &[]);
/// assert_eq!(codetether_evidence_ledger::ledger::unclassified_count(&ledger, "done"), 0);
/// ```
pub fn unclassified_count(ledger: &ScopeLedger, answer: &str) -> usize {
    let lower = answer.to_ascii_lowercase();
    ledger
        .items
        .iter()
        .filter(|item| !lower.contains(&item.deliverable.to_ascii_lowercase()))
        .count()
}

fn status_for(evidence: &[codetether_evidence_core::record::EvidenceRecord]) -> String {
    if evidence.iter().any(|ev| ev.level == "blocked") {
        "blocked".to_string()
    } else if evidence.is_empty() {
        "pending".to_string()
    } else {
        "proven".to_string()
    }
}
