//! Bind optional forage scoring to application authorization and plugin execution.
use codetether_okr::{KeyResult, Okr};
mod host;
pub(super) fn apply(
    okr: &Okr,
    kr: &KeyResult,
    base_score: f64,
    remaining: f64,
    moonshot_alignment: f64,
) -> f64 {
    codetether_forage_scoring::apply::<host::Native>(
        okr,
        kr,
        base_score,
        remaining,
        moonshot_alignment,
    )
}
