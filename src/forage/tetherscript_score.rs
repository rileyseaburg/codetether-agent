#[path = "score_input.rs"]
mod score_input;
pub(crate) use score_input::ScoreInput;

use crate::okr::{KeyResult, Okr};

#[cfg(not(feature = "tetherscript"))]
mod disabled;
#[cfg(feature = "tetherscript")]
mod enabled;

#[cfg(not(feature = "tetherscript"))]
use disabled::score_with_tetherscript;
#[cfg(feature = "tetherscript")]
use enabled::score_with_tetherscript;

pub(crate) fn apply(
    okr: &Okr,
    kr: &KeyResult,
    base_score: f64,
    remaining: f64,
    moonshot_alignment: f64,
) -> f64 {
    let input = ScoreInput {
        okr,
        kr,
        base_score,
        remaining,
        moonshot_alignment,
    };
    score_with_tetherscript(&input).unwrap_or(input.base_score)
}
