//! Bind application score plugins and interactive goal clarification.
use super::Native;
use crate::okr::{KeyResult, Okr, OkrRepository};
impl codetether_forage_runtime::Host for Native {
    fn score(
        okr: &Okr,
        kr: &KeyResult,
        base_score: f64,
        remaining: f64,
        moonshot_alignment: f64,
    ) -> f64 {
        crate::forage::tetherscript_score::apply(okr, kr, base_score, remaining, moonshot_alignment)
    }
    async fn clarify(repo: &OkrRepository, okr_count: usize) -> anyhow::Result<bool> {
        crate::forage::clarify::clarify_and_persist(repo, okr_count).await
    }
}
