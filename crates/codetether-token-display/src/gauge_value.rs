//! Context-percentage calculation and named-color fallback.
use super::context_gauge::CELLS;
use ratatui::{
    style::{Color, Style},
    text::Span,
};

pub(super) fn gauge_pct(used: Option<usize>, budget: Option<usize>) -> Option<usize> {
    let (used, budget) = (used?, budget?);
    if budget == 0 {
        return None;
    }
    Some(((used as f64 / budget as f64) * 100.0).clamp(0.0, 100.0) as usize)
}

pub(super) fn flat_gauge_span(filled: usize, pct: usize) -> Span<'static> {
    let bar: String = (0..CELLS)
        .map(|i| if i < filled { '█' } else { '░' })
        .collect();
    let color = match pct {
        0..=70 => Color::Green,
        71..=90 => Color::Yellow,
        _ => Color::Red,
    };
    Span::styled(format!("▕{bar}▏ {pct}%"), Style::default().fg(color))
}
