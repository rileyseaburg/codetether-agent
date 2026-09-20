//! Draw one sequence-diagram message arrow between two lifelines.

use ratatui::style::Color;

use crate::tui::chat::mermaid::canvas::Canvas;
use crate::tui::chat::mermaid::model::Edge;

/// Draw the arrow shaft, head, and optional label on row `y`.
pub fn draw(canvas: &mut Canvas, edge: &Edge, from: usize, to: usize, y: usize, color: Color) {
    if from == to {
        canvas.put(from, y, "↺", color);
        label(canvas, edge, from + 2, y);
        return;
    }
    let (lo, hi) = (from.min(to), from.max(to));
    let body = if edge.dotted { '╌' } else { '─' };
    for x in lo + 1..hi {
        canvas.set(x, y, body, color);
    }
    if to > from {
        canvas.set(hi, y, '▶', color);
    } else {
        canvas.set(lo, y, '◀', color);
    }
    label(canvas, edge, lo + 2, y.saturating_sub(1));
}

fn label(canvas: &mut Canvas, edge: &Edge, x: usize, y: usize) {
    if let Some(text) = &edge.label {
        let room = canvas.width().saturating_sub(x);
        let clipped: String = text.chars().take(room).collect();
        canvas.put(x, y, &clipped, Color::Yellow);
    }
}
