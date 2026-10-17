//! Story completion metrics shared while updating each key result.

use codetether_okr::KrOutcomeType;
pub(crate) struct Metrics {
    pub(crate) passed: usize,
    pub(crate) total: usize,
    pub(crate) ratio: f64,
    pub(crate) outcome_type: KrOutcomeType,
    pub(crate) story_evidence: Vec<String>,
}
