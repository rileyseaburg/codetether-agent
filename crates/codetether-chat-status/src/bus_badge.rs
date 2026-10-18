//! Styled compact bus badge.
use crate::{BusStatus, bus_status_label_and_color};
use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

/// Build a compact agent-bus badge.
///
/// # Arguments
/// * `status` - Connectivity and task-count snapshot.
///
/// # Returns
/// An owned, bold status span.
///
/// # Examples
/// ```
/// let _badge = codetether_chat_status::bus_status_badge_span(Default::default());
/// ```
pub fn bus_status_badge_span(status: BusStatus) -> Span<'static> {
    let (label, color) = bus_status_label_and_color(status);
    Span::styled(
        format!(" {label} "),
        Style::default().fg(color).add_modifier(Modifier::BOLD),
    )
}
