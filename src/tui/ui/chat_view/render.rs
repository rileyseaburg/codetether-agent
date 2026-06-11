//! Chat view orchestrator.
//!
//! [`render_chat_view`] is the single entry-point called from
//! [`crate::tui::ui::main`]. Composes layout, lines, input, and status.

use ratatui::Frame;

use super::attachment::attachment_suffix;
use super::input_area::render_input;
use super::layout_compute::compute_chat_chunks;
use super::lines::build_chat_lines;
use super::messages::render_messages_ref;
use super::status_line::render_status_line;
use super::suggestions::render_suggestions;
use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::message_formatter::MessageFormatter;

/// Top-level chat view renderer.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::render_chat_view;
/// # fn d(f:&mut ratatui::Frame,a:&mut codetether_agent::tui::app::state::App){
/// # let s = codetether_agent::tui::app::session_runtime::SessionView::default();
/// # render_chat_view(f,a,&s); }
/// ```
pub fn render_chat_view(f: &mut Frame, app: &mut App, session: &SessionView) {
    let chunks = compute_chat_chunks(f.area(), app);
    let palette = ColorPalette::marketing();
    let formatter = MessageFormatter::new(chunks.messages.width.saturating_sub(4) as usize);
    let max_width = chunks.messages.width as usize;
    let drawn = build_chat_lines(app, max_width, max_width, &formatter, &palette);
    render_messages_ref(f, app, session, &chunks, &palette, drawn.as_slice());
    drawn.restore(app, max_width);
    let suffix = attachment_suffix(app);
    render_input(f, app, chunks.input, &palette, &suffix);
    if let Some(rect) = chunks.suggestions {
        render_suggestions(f, app, rect);
    }
    render_status_line(f, app, chunks.status);
    super::approval_overlay::render(f, f.area());
    crate::tui::help::render_help_overlay_if_needed(f, &mut app.state);
}
