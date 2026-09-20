//! Fixed-size character grid used to compose mermaid diagrams.
//!
//! Diagram renderers draw boxes and connectors into a [`Canvas`] and then
//! convert it into ratatui [`Line`]s once, which keeps layout math separate
//! from styling concerns.

use ratatui::style::Color;

/// One grid cell: a character plus its foreground color.
pub type Cell = (char, Color);

/// A rectangular character grid with per-cell color.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::chat::mermaid::canvas::Canvas;
/// use ratatui::style::Color;
///
/// let mut c = Canvas::new(6, 1);
/// c.put(0, 0, "hi", Color::Cyan);
/// assert_eq!(c.to_lines().len(), 1);
/// ```
pub struct Canvas {
    width: usize,
    pub(super) cells: Vec<Vec<Cell>>,
}

impl Canvas {
    /// Create a blank canvas of `width` columns and `height` rows.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            cells: vec![vec![(' ', Color::Reset); width]; height],
        }
    }

    /// Grid width in columns.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Write one character, ignoring out-of-bounds coordinates.
    pub fn set(&mut self, x: usize, y: usize, ch: char, color: Color) {
        if let Some(cell) = self.cells.get_mut(y).and_then(|r| r.get_mut(x)) {
            *cell = (ch, color);
        }
    }

    /// Write a string starting at `x`, clipping at the right edge.
    pub fn put(&mut self, x: usize, y: usize, text: &str, color: Color) {
        for (i, ch) in text.chars().enumerate() {
            self.set(x + i, y, ch, color);
        }
    }
}
