//! Native score plugins and interactive goal clarification.
use super::{Execution, Host};
use codetether_okr::{KeyResult, Okr, OkrRepository};
impl<E: Execution> codetether_forage_runtime::Host for Host<E> {
    fn score(
        okr: &Okr,
        kr: &KeyResult,
        base_score: f64,
        remaining: f64,
        moonshot_alignment: f64,
    ) -> f64 {
        crate::scoring::apply(okr, kr, base_score, remaining, moonshot_alignment)
    }
    async fn clarify(repo: &OkrRepository, okr_count: usize) -> anyhow::Result<bool> {
        codetether_forage_clarify::clarify_and_persist(repo, okr_count).await
    }
}
