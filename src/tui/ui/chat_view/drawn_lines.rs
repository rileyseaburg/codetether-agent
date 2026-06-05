//! Drawn chat-line cache wrapper.

use ratatui::text::Line;

use crate::tui::app::state::App;

/// Rendered chat lines plus whether they must be restored to the cache.
pub struct DrawnLines {
    lines: Vec<Line<'static>>,
    from_cache: bool,
}

impl DrawnLines {
    /// Create lines taken from the cache.
    pub(crate) fn from_cache(lines: Vec<Line<'static>>) -> Self {
        Self {
            lines,
            from_cache: true,
        }
    }

    /// Create lines rebuilt for the current frame.
    pub(crate) fn from_rebuild(lines: Vec<Line<'static>>) -> Self {
        Self {
            lines,
            from_cache: false,
        }
    }

    /// Access the lines for rendering.
    pub fn as_slice(&self) -> &[Line<'static>] {
        &self.lines
    }

    /// Restore cached lines after rendering.
    pub fn restore(self, app: &mut App, _max_width: usize) {
        if self.from_cache {
            app.state.restore_cached_message_lines(self.lines);
        }
    }
}
