//! Application scoring and interactive clarification contracts.
use anyhow::Result;
use codetether_okr::{KeyResult, Okr, OkrRepository};
use std::future::Future;
/// Application dependencies retained outside the shared forage runtime.
/// # Implementors
/// Implement the execution host, custom score hook and interactive goal prompt.
pub trait Host: codetether_forage_execution::Host {
    /// Apply the existing custom scoring hook without changing input order.
    fn score(
        okr: &Okr,
        kr: &KeyResult,
        base_score: f64,
        remaining: f64,
        moonshot_alignment: f64,
    ) -> f64;
    /// Ask for and persist goals when an empty queue can be clarified.
    fn clarify(repo: &OkrRepository, okr_count: usize)
    -> impl Future<Output = Result<bool>> + Send;
}
