//! Help blank content and presentation.
use ratatui::text::Line;
pub(crate) fn blank() -> Line<'static> {
    Line::from("")
}
