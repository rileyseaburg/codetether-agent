//! Pending approval popup for the chat view.

use crate::tui::app::state::{App, approval_queue};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Borders, Clear},
};

pub(crate) fn render(f: &mut Frame, app: &mut App, area: Rect) {
    if approval_queue::feedback_input(&app.state.input) {
        return;
    }
    let Some(item) = approval_queue::active() else {
        return;
    };
    let popup = super::approval_overlay_layout::popup(area);
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Approval · {} pending", approval_queue::len()));
    let inner = block.inner(popup);
    let footer_height =
        super::approval_overlay_footer::height(inner.width).min(inner.height.saturating_sub(3));
    let [preview, footer] =
        Layout::vertical([Constraint::Min(3), Constraint::Length(footer_height)]).areas(inner);
    f.render_widget(Clear, popup);
    f.render_widget(block, popup);
    super::approval_overlay_preview::render(
        f,
        preview,
        &item,
        &mut app.state.approval_preview_scroll,
    );
    super::approval_overlay_footer::render(f, footer);
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
