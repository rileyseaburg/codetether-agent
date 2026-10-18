//! Session picker layout orchestration.
//!
//! Header, visible-list, and help-bar rendering live in focused child modules.

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
};

use crate::{PickerView, header, help, list};
use ratatui::text::Span;

/// Draw the session picker and its supplied bus badge.
///
/// # Arguments
/// * `f` - Frame receiving the full picker layout.
/// * `view` - Borrowed sessions, selection, filter, and status.
/// * `bus_badge` - Application-provided bus status renderer.
///
/// # Returns
/// Draws into the frame without modifying application state.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, view: &codetether_session_picker::PickerView<'_>) {
/// codetether_session_picker::render_sessions_view(f, view, || "BUS offline".into());
/// # }
/// ```
pub fn render_sessions_view(
    f: &mut Frame,
    view: &PickerView<'_>,
    bus_badge: impl FnOnce() -> Span<'static>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
        ])
        .split(f.area());
    header::render(f, view, chunks[0]);
    list::render(f, view, chunks[1]);
    help::render(f, bus_badge(), chunks[2]);
}
