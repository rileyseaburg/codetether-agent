//! Click hit-testing data for the chat messages panel.
//!
//! Each frame the messages renderer records the panel rect, the applied
//! vertical scroll offset, and the plain text of every rendered line. A
//! mouse click is then mapped back to the line under the cursor so the
//! click-to-open handler can extract a file path from that line.

use ratatui::layout::Rect;
use ratatui::text::Line;

/// Recorded geometry + plain text of the last rendered chat panel.
#[derive(Debug, Clone, Default)]
pub struct ChatHit {
    /// Panel rect including borders (interior starts at `y + 1`).
    pub rect: Rect,
    /// Vertical scroll offset applied to the paragraph this frame.
    pub scroll: u16,
    /// Plain text of each rendered chat line, top to bottom.
    pub lines: Vec<String>,
}

impl ChatHit {
    /// Records the rendered geometry and flattens `lines` to plain text.
    pub fn record(&mut self, rect: Rect, scroll: u16, lines: &[Line<'static>]) {
        self.rect = rect;
        self.scroll = scroll;
        self.lines.clear();
        self.lines.extend(lines.iter().map(line_text));
    }

    /// Returns the text of the chat line under screen cell `(col, row)`.
    ///
    /// Returns `None` when the click falls on a border or outside the panel.
    pub fn line_at(&self, col: u16, row: u16) -> Option<&str> {
        let r = self.rect;
        if col <= r.x || col >= r.x + r.width.saturating_sub(1) {
            return None;
        }
        if row <= r.y || row >= r.y + r.height.saturating_sub(1) {
            return None;
        }
        let interior = row - r.y - 1;
        let idx = interior as usize + self.scroll as usize;
        self.lines.get(idx).map(String::as_str)
    }
}

/// Flattens a styled [`Line`] into its plain-text content.
fn line_text(line: &Line<'static>) -> String {
    line.spans.iter().map(|s| s.content.as_ref()).collect()
}
