//! Wide and stacked status assembly from prepared badge groups.
use crate::{STACK_WIDTH_THRESHOLD, status_hints::keybinding_spans};
use ratatui::text::{Line, Span};

/// Assemble prepared badge and metric groups into the responsive status bar.
///
/// # Arguments
/// * `badges` - Prepared badge spans.
/// * `metrics` - Prepared metric spans.
/// * `width` - Available terminal width.
///
/// # Returns
/// Owned terminal spans or lines; optional badges are absent when inactive.
///
/// # Examples
/// ```
/// let _display = codetether_chat_status::build_status_lines(Vec::new(), Vec::new(), 180);
/// ```
pub fn build_status_lines(
    badges: Vec<Span<'static>>,
    metrics: Vec<Span<'static>>,
    width: u16,
) -> Vec<Line<'static>> {
    if width >= STACK_WIDTH_THRESHOLD {
        let mut combined = keybinding_spans();
        combined.push(Span::raw(" | "));
        combined.extend(badges);
        combined.push(Span::raw(" | "));
        combined.extend(metrics);
        vec![Line::from(combined)]
    } else {
        codetether_chat_layout::pack_stacked(
            super::compact_hints::compact_keybinding_spans(),
            badges,
            metrics,
            width,
        )
    }
}
