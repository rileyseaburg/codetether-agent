//! Optional TetherScript scoring for forage opportunities.
//!
//! [apply] preserves the base score when scripting is disabled, no plugin is
//! present, authorization denies execution, or the plugin returns no score.
//! Input construction and result decoding are separated from execution. [Host]
//! supplies the application policy and concrete tool only in scripting builds.
mod host;
pub use host::Host;
#[cfg(feature = "tetherscript")]
mod event;
mod score_input;
#[cfg(feature = "tetherscript")]
mod score_value;
pub(crate) use score_input::ScoreInput;

use codetether_okr::{KeyResult, Okr};

#[cfg(not(feature = "tetherscript"))]
mod disabled;
#[cfg(feature = "tetherscript")]
mod enabled;

#[cfg(not(feature = "tetherscript"))]
use disabled::score_with_tetherscript;
#[cfg(feature = "tetherscript")]
use enabled::score_with_tetherscript;

/// Apply the optional score plugin, preserving the base score on fallback.
/// # Arguments
/// * `okr` — Objective associated with this opportunity.
/// * `kr` — Key result being scored.
/// * `base_score` — Score returned when no plugin value is available.
/// * `remaining` — Fraction of key-result work remaining.
/// * `moonshot_alignment` — Alignment score passed to the plugin.
/// # Returns
/// The plugin's numeric score, or the original base score.
/// # Panics
/// With scripting enabled and a score file present, retains Tokio's
/// `block_in_place` runtime requirements.
/// # Examples
/// ```no_run
/// # fn score<H: codetether_forage_scoring::Host>(okr: &codetether_okr::Okr, kr: &codetether_okr::KeyResult) {
/// let value = codetether_forage_scoring::apply::<H>(okr, kr, 0.5, 0.8, 0.4);
/// # let _ = value;
/// # }
/// ```
pub fn apply<H: Host>(
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
    score_with_tetherscript::<H>(&input).unwrap_or(input.base_score)
}
