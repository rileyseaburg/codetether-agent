//! Preserve the base score when scripting is not compiled in.
use super::{Host, ScoreInput};

pub(super) fn score_with_tetherscript<H: Host>(_input: &ScoreInput<'_>) -> Option<f64> {
    None
}
