//! Assemble a scored opportunity record while preserving source metadata.
use crate::ForageOpportunity;
use codetether_okr::{KeyResult, Okr};
pub(super) fn build(
    okr: &Okr,
    kr: &KeyResult,
    (score, progress, remaining): (f64, f64, f64),
    (moonshot_alignment, moonshot_hits): (f64, Vec<String>),
    prompt: String,
) -> ForageOpportunity {
    ForageOpportunity {
        score,
        okr_id: okr.id,
        okr_title: okr.title.clone(),
        okr_status: okr.status,
        key_result_id: kr.id,
        key_result_title: kr.title.clone(),
        progress,
        remaining,
        target_date: okr.target_date,
        moonshot_alignment,
        moonshot_hits,
        prompt,
    }
}
