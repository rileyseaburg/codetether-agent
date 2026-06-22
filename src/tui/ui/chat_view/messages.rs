//! Scrollable messages panel renderer.
//!
//! Composes the block title (via [`build_title`]), scroll clamping (via
//! [`clamp_scroll`]), and the wrapped [`Paragraph`] widget.

use ratatui::{Frame, text::Line, widgets::Paragraph};

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::ui::border_style::pane_border;
use crate::tui::ui::mode_accent::mode_accent;

use super::layout_chunks::ChatChunks;
use super::scroll::clamp_scroll;
use super::title::build_title;

/// Render the chat messages block with title, borders, and scroll.
///
/// Takes lines by value. For a zero-clone variant, see [`render_messages_ref`].
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::messages::render_messages;
/// # fn demo(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App) {
/// let pal = codetether_agent::tui::color_palette::ColorPalette::marketing();
/// let chunks = codetether_agent::tui::ui::chat_view::layout_compute::compute_chat_chunks(f.area(), app);
/// let view = codetether_agent::tui::app::session_runtime::SessionView::default();
/// render_messages(f, app, &view, &chunks, &pal, vec![]);
/// # }
/// ```
#[allow(dead_code)]
pub fn render_messages(
    f: &mut Frame,
    app: &mut App,
    session: &SessionView,
    chunks: &ChatChunks,
    _palette: &ColorPalette,
    lines: Vec<Line<'static>>,
) {
    let accent = mode_accent(&app.state.view_mode);
    let block = pane_border(true)
        .border_style(ratatui::style::Style::default().fg(accent))
        .title(build_title(app, session));
    let scroll = clamp_scroll(app, chunks.messages, &lines);
    let chat = Paragraph::new(lines).block(block).scroll((scroll, 0));
    f.render_widget(chat, chunks.messages);
}

/// Zero-clone render: takes a `&[Line]` reference instead of `Vec<Line>`.
///
/// This avoids cloning the entire line buffer when the caller intends to
/// reuse the lines after rendering (e.g. caching for the next frame).
pub fn render_messages_ref(
    f: &mut Frame,
    app: &mut App,
    session: &SessionView,
    chunks: &ChatChunks,
    _palette: &ColorPalette,
    lines: &[Line<'static>],
) {
    let accent = mode_accent(&app.state.view_mode);
    let block = pane_border(true)
        .border_style(ratatui::style::Style::default().fg(accent))
        .title(build_title(app, session));
    let scroll = clamp_scroll(app, chunks.messages, lines);
    app.state.chat_hit.record(chunks.messages, scroll, lines);
    let chat = Paragraph::new(lines.to_vec())
        .block(block)
        .scroll((scroll, 0));
    f.render_widget(chat, chunks.messages);
    super::scrollbar_render::render_scrollbar(f, chunks.messages, lines.len(), scroll as usize);
}
