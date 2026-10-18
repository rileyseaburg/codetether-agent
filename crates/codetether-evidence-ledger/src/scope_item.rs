//! Serializable deliverable status and bounded evidence references.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
/// One deliverable with its status and bounded supporting evidence.
/// # Examples
/// ```
/// let item = codetether_evidence_ledger::scope_item::ScopeItem::new("review".into(), "pending".into(), &[]);
/// assert!(item.evidence.is_empty());
/// ```
pub struct ScopeItem {
    /// User-requested deliverable text.
    pub deliverable: String,
    /// Reported completion status.
    pub status: String,
    /// Level from the first available observation.
    pub evidence_level: Option<String>,
    /// At most five observation values.
    pub evidence: Vec<String>,
}

impl ScopeItem {
    /// Build an item from its deliverable, status and ordered observations.
    pub fn new(
        deliverable: String,
        status: String,
        evidence: &[codetether_evidence_core::record::EvidenceRecord],
    ) -> Self {
        Self {
            deliverable,
            status,
            evidence_level: evidence.first().map(|ev| ev.level.clone()),
            evidence: evidence.iter().map(|ev| ev.value.clone()).take(5).collect(),
        }
    }
}
