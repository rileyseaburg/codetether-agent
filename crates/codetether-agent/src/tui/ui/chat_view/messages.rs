//! Native entry points for shared message-panel rendering.
mod host;
use super::layout_chunks::ChatChunks;
use crate::tui::{
    app::{session_runtime::SessionView, state::App},
    color_palette::ColorPalette,
};
use ratatui::{Frame, text::Line};
/// Draw an owned message buffer using the shared panel renderer.
#[allow(dead_code)]
pub fn render_messages(
    f: &mut Frame,
    app: &mut App,
    session: &SessionView,
    chunks: &ChatChunks,
    _palette: &ColorPalette,
    lines: Vec<Line<'static>>,
) {
    codetether_chat_layout::messages::render_owned(
        f,
        &mut host::Native { app, session },
        chunks.messages,
        lines,
    );
}
/// Draw borrowed messages and update visible-line hit targets.
pub fn render_messages_ref(
    f: &mut Frame,
    app: &mut App,
    session: &SessionView,
    chunks: &ChatChunks,
    _palette: &ColorPalette,
    lines: &[Line<'static>],
) {
    codetether_chat_layout::messages::render_ref(
        f,
        &mut host::Native { app, session },
        chunks.messages,
        lines,
    );
}
