//! Pending approval popup for the chat view.

use crate::tui::app::state::App;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Clear},
};
pub(crate) fn render(f: &mut ratatui::Frame, app: &mut App, area: ratatui::layout::Rect) {
    codetether_approval_view::render(
        f,
        area,
        &app.state.composer.input,
        &mut app.state.approval_preview_scroll,
    );
}

#[cfg(test)]
#[path = "approval_overlay/content_tests.rs"]
mod content_tests;
#[cfg(test)]
#[path = "approval_overlay/render_tests.rs"]
mod render_tests;
#[cfg(test)]
#[path = "approval_overlay/test_support.rs"]
mod test_support;
