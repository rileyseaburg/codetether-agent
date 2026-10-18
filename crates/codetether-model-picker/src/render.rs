//! Model picker panel layout.
use crate::{PickerView, matching};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
};
/// Render filtering status, catalog, selection and keyboard controls.
/// # Arguments
/// * `f` - Target terminal frame.
/// * `area` - Available screen rectangle.
/// * `view` - Catalog, query, selected index and session fallback.
/// # Returns
/// Unit after rendering all four sections.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect, view: &codetether_model_picker::PickerView<'_>) {
/// codetether_model_picker::render_model_picker(f, area, view);
/// # }
/// ```
pub fn render_model_picker(f: &mut Frame, area: Rect, view: &PickerView<'_>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(3),
            Constraint::Length(3),
        ])
        .split(area);

    super::header::render(f, view, chunks[0]);
    let models = matching(view.catalog, view.filter);
    super::list::render(f, view, &models, chunks[1]);
    super::selection::render(f, view, &models, chunks[2]);
    super::footer::render(f, chunks[3]);
}
