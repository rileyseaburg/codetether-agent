//! Drawn chat-line cache wrapper.

use ratatui::text::Line;

use crate::RenderedLineCache;

/// Rendered chat lines plus whether they must be restored to the cache.
///
/// # Examples
/// ```
/// use codetether_message_cache::{DrawnLines, RenderedLineCache};
/// let drawn = DrawnLines::from_cache(vec![ratatui::text::Line::from("text")]);
/// let mut cache = RenderedLineCache::default();
/// drawn.restore(&mut cache, 80);
/// assert_eq!(cache.cached_message_lines.len(), 1);
/// ```
pub struct DrawnLines {
    lines: Vec<Line<'static>>,
    from_cache: bool,
}

impl DrawnLines {
    /// Create lines taken from the cache.
    pub fn from_cache(lines: Vec<Line<'static>>) -> Self {
        Self {
            lines,
            from_cache: true,
        }
    }

    /// Create lines rebuilt for the current frame.
    pub fn from_rebuild(lines: Vec<Line<'static>>) -> Self {
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
    pub fn restore(self, cache: &mut RenderedLineCache, _max_width: usize) {
        if self.from_cache {
            cache.restore_cached_message_lines(self.lines);
        }
    }
}
