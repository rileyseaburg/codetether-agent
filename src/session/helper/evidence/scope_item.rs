use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ScopeItem {
    pub deliverable: String,
    pub status: String,
    pub evidence_level: Option<String>,
    pub evidence: Vec<String>,
}

impl ScopeItem {
    pub(crate) fn new(
        deliverable: String,
        status: String,
        evidence: &[super::record::EvidenceRecord],
    ) -> Self {
        Self {
            deliverable,
            status,
            evidence_level: evidence.first().map(|ev| ev.level.clone()),
            evidence: evidence.iter().map(|ev| ev.value.clone()).take(5).collect(),
        }
    }
}
