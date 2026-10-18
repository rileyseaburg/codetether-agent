//! Numeric and coordinate-pair bounds.
use super::{A, ComputerUseInput};
use anyhow::{Result, ensure};

pub(super) fn check(input: &ComputerUseInput) -> Result<()> {
    for v in [input.x, input.y, input.x2, input.y2].into_iter().flatten() {
        let valid =
            v.is_finite() && v.fract() == 0.0 && v >= i32::MIN as f64 && v <= i32::MAX as f64;
        ensure!(
            valid,
            "Shadow coordinates must be finite integral i32 values"
        );
    }
    ensure!(
        input.x.is_some() == input.y.is_some() && input.x2.is_some() == input.y2.is_some(),
        "Coordinates require x/y pairs"
    );
    ensure!(
        (1..=240).contains(&input.steps.unwrap_or(12)),
        "Shadow steps must be 1..=240"
    );
    ensure!(
        input.duration_ms.unwrap_or(500) <= 30_000,
        "Shadow duration_ms must be <= 30000"
    );
    if matches!(input.action, A::Scroll) {
        let delta = input
            .scroll_amount
            .ok_or_else(|| anyhow::anyhow!("scroll_amount is required"))?;
        ensure!(
            i16::try_from(delta).is_ok() && delta != 0,
            "Wheel delta must be nonzero signed 16-bit"
        );
    }
    if matches!(input.action, A::Drag) {
        ensure!(input.x2.is_some(), "Shadow drag requires x2/y2");
    }
    Ok(())
}
