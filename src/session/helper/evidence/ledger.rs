use serde::{Deserialize, Serialize};

use crate::session::Session;

use super::scope_item::ScopeItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ScopeLedger {
    pub session_id: String,
    pub items: Vec<ScopeItem>,
    pub evidence: Vec<super::record::EvidenceRecord>,
}

pub(crate) fn build(session: &Session) -> ScopeLedger {
    let evidence = super::extract::from_text(&super::message_text::joined(&session.messages));
    let status = status_for(&evidence);
    let items = super::scope_extract::from_messages(&session.messages)
        .into_iter()
        .map(|item| ScopeItem::new(item, status.clone(), &evidence))
        .collect();
    ScopeLedger {
        session_id: session.id.clone(),
        items,
        evidence,
    }
}

pub(crate) fn unclassified_count(ledger: &ScopeLedger, answer: &str) -> usize {
    let lower = answer.to_ascii_lowercase();
    ledger
        .items
        .iter()
        .filter(|item| !lower.contains(&item.deliverable.to_ascii_lowercase()))
        .count()
}

fn status_for(evidence: &[super::record::EvidenceRecord]) -> String {
    if evidence.iter().any(|ev| ev.level == "blocked") {
        "blocked".to_string()
    } else if evidence.is_empty() {
        "pending".to_string()
    } else {
        "proven".to_string()
    }
}
