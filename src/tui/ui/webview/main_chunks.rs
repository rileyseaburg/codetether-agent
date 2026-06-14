//! Vertical layout for the webview: header, body, input, suggestions, status.

use ratatui::layout::{Constraint, Direction, Layout, Rect};

use crate::tui::app::state::App;

/// Webview main rectangles, including the optional suggestions row.
pub(super) struct MainChunks {
    pub header: Rect,
    pub body: Rect,
    pub input: Rect,
    pub suggestions: Option<Rect>,
    pub status: Rect,
}

/// Split `area` vertically. Input height adapts to typed lines (3–6 rows)
/// and a 5-row suggestions panel appears while slash autocomplete is open,
/// matching the classic chat layout.
pub(super) fn compute(area: Rect, app: &App) -> MainChunks {
    let suggestions_visible = app.state.slash_suggestions_visible();
    let input_height = (app.state.input.lines().count().max(1) as u16 + 2).clamp(3, 6);
    let mut constraints = vec![
        Constraint::Length(3),
        Constraint::Min(1),
        Constraint::Length(input_height),
    ];
    if suggestions_visible {
        constraints.push(Constraint::Length(5));
    }
    constraints.push(Constraint::Length(1));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints)
        .split(area);
    MainChunks {
        header: chunks[0],
        body: chunks[1],
        input: chunks[2],
        suggestions: suggestions_visible.then(|| chunks[3]),
        status: chunks[if suggestions_visible { 4 } else { 3 }],
    }
}
