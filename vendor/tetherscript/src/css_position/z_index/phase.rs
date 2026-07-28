//! Paint phase classification.

use super::PaintPhase;

pub fn classify(positioned: bool, z: i32) -> PaintPhase {
    if positioned && z < 0 {
        PaintPhase::NegativePositioned
    } else if !positioned {
        PaintPhase::NormalFlow
    } else if z == 0 {
        PaintPhase::AutoOrZeroPositioned
    } else {
        PaintPhase::PositivePositioned
    }
}

pub fn rank(phase: PaintPhase) -> i32 {
    match phase {
        PaintPhase::NegativePositioned => 0,
        PaintPhase::NormalFlow => 1,
        PaintPhase::AutoOrZeroPositioned => 2,
        PaintPhase::PositivePositioned => 3,
    }
}
