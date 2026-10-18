//! Borrowed opportunity fields passed to the score plugin.
use codetether_okr::{KeyResult, Okr};

pub(crate) struct ScoreInput<'a> {
    pub okr: &'a Okr,
    pub kr: &'a KeyResult,
    pub base_score: f64,
    pub remaining: f64,
    pub moonshot_alignment: f64,
}
