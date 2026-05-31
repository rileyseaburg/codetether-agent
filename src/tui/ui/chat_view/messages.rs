//! Scrollable messages panel renderer.
//!
//! Composes the block title (via [`build_title`]), scroll clamping (via
//! [`clamp_scroll`]), and the wrapped [`Paragraph`] widget.

use ratatui::{
    Frame,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph},
};

use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;

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
/// # fn demo(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App, sess: &codetether_agent::session::Session) {
/// let pal = codetether_agent::tui::color_palette::ColorPalette::marketing();
/// let chunks = codetether_agent::tui::ui::chat_view::layout_compute::compute_chat_chunks(f.area(), app);
/// render_messages(f, app, sess, &chunks, &pal, vec![]);
/// # }
/// ```
#[allow(dead_code)]
pub fn render_messages(
    f: &mut Frame,
    app: &mut App,
    session: &Session,
    chunks: &ChatChunks,
    palette: &ColorPalette,
    lines: Vec<Line<'static>>,
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border))
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
    session: &Session,
    chunks: &ChatChunks,
    palette: &ColorPalette,
    lines: &[Line<'static>],
) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(palette.border))
        .title(build_title(app, session));
    let scroll = clamp_scroll(app, chunks.messages, lines);
    let chat = Paragraph::new(lines.to_vec()).block(block).scroll((scroll, 0));
    f.render_widget(chat, chunks.messages);
}
