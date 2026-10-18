//! Inline context-budget gauge for the status bar.
//!
//! Renders the percentage of the model context window consumed as a small
//! 8-cell bar. On truecolor terminals each cell is colored along a
//! green → red heat gradient (see [`context_gauge_heat`]).
//! On 8/256-color terminals a flat solid-color bar is used instead.

use ratatui::text::Span;

use super::gauge_value::{flat_gauge_span, gauge_pct};
use codetether_theme::gradient::rgb_supported;

#[path = "context_gauge_heat.rs"]
pub mod context_gauge_heat;

pub(super) const CELLS: usize = 8;

/// Build a gradient context-budget gauge, or `None` if no estimate yet.
///
/// # Examples
///
/// ```
/// let spans = codetether_token_display::context_gauge::context_gauge_spans(Some(40), Some(100));
/// assert!(spans.is_some());
/// ```
///
/// # Arguments
/// * `used` - Used context tokens.
/// * `budget` - Available context tokens.
///
/// # Returns
/// Styled gauge spans, or none without a nonzero budget.
pub fn context_gauge_spans(
    used: Option<usize>,
    budget: Option<usize>,
) -> Option<Vec<Span<'static>>> {
    let pct = gauge_pct(used, budget)?;
    let filled = (pct * CELLS) / 100;
    if rgb_supported() {
        Some(context_gauge_heat::heat_gradient_spans(filled, CELLS, pct))
    } else {
        Some(vec![flat_gauge_span(filled, pct)])
    }
}

/// Single-span gauge for non-truecolor terminals.
///
/// # Arguments
/// * `used` - Used context tokens.
/// * `budget` - Available context tokens.
///
/// # Returns
/// A single named-color gauge span, or none without a nonzero budget.
///
/// # Examples
/// ```
/// assert!(codetether_token_display::context_gauge::context_gauge_span(Some(5),Some(10)).is_some());
/// ```
pub fn context_gauge_span(used: Option<usize>, budget: Option<usize>) -> Option<Span<'static>> {
    let pct = gauge_pct(used, budget)?;
    Some(flat_gauge_span((pct * CELLS) / 100, pct))
}
