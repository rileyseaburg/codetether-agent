//! Pending approval popup for the chat view.

use ratatui::widgets::{Block, Borders, Clear};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::tui::app::state::{App, approval_queue};

pub(crate) fn render(f: &mut Frame, app: &App, area: Rect) {
    if approval_queue::feedback_input(&app.state.input) {
        return;
    }
    let Some(item) = approval_queue::active() else {
        return;
    };
    let popup = super::approval_overlay_layout::popup(area);
    let block = Block::default().borders(Borders::ALL).title("Approval");
    let inner = block.inner(popup);
    let lsp_height = super::approval_overlay_lsp::height(&item.report);
    let [header, preview, lsp, footer] = Layout::vertical([
        Constraint::Length(2),
        Constraint::Min(1),
        Constraint::Length(lsp_height),
        Constraint::Length(2),
    ])
    .areas(inner);
    f.render_widget(Clear, popup);
    f.render_widget(block, popup);
    super::approval_overlay_text::header(f, header, &item);
    super::approval_overlay_preview::render(
        f,
        preview,
        item.preview.as_deref(),
        &item.resource,
        app.state.approval_preview_scroll,
    );
    super::approval_overlay_lsp::render(f, lsp, &item.report);
    super::approval_overlay_text::footer(f, footer, &item, approval_queue::len());
}
