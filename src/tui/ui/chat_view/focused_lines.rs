//! Active-agent transcript selection before the main-chat cache path.

use super::drawn_lines::DrawnLines;
use crate::tui::app::state::App;

pub(super) fn build(
    app: &mut App,
    content_width: usize,
    formatter: &crate::tui::message_formatter::MessageFormatter,
    palette: &crate::tui::color_palette::ColorPalette,
) -> Option<DrawnLines> {
    if let Some(lines) = super::swarm_lines::build(app, content_width, formatter, palette) {
        return Some(lines);
    }
    super::agent_lines::build(app)
}
