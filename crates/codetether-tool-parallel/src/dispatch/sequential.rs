//! Ordered fallback for a planned call range.
use super::{Host, ToolCall};
use std::ops::Range;
pub(super) async fn run(
    host: &mut impl Host,
    step: usize,
    calls: &[ToolCall],
    range: Range<usize>,
) -> anyhow::Result<bool> {
    for (id, name, input) in &calls[range] {
        if host.single(step, id, name, input).await? {
            return Ok(true);
        }
    }
    Ok(false)
}
