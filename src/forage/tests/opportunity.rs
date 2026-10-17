//! Common execution evidence opportunity fixture.
use super::*;
pub(super) fn new(okr_id: Uuid, kr_id: Uuid, progress: f64, remaining: f64) -> ForageOpportunity {
    ForageOpportunity {
        score: 1.35,
        okr_id,
        okr_title: "Autonomous Business-Aligned Execution".to_string(),
        okr_status: OkrStatus::Active,
        key_result_id: kr_id,
        key_result_title: "KR1".to_string(),
        progress,
        remaining,
        target_date: None,
        moonshot_alignment: 0.0,
        moonshot_hits: Vec::new(),
        prompt: "test prompt".to_string(),
    }
}
