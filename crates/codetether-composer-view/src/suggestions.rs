//! Slash-command suggestion rows and current selection.
use crate::suggestion_style::highlight_style;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};
/// Render command completions or a lazily supplied usage hint.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `suggestions` - Matching command strings.
/// * `selected` - Selected completion index.
/// * `area` - Completion-panel rectangle.
/// * `usage_hint` - Called only when the command list is empty.
///
/// # Returns
/// Draws matching commands or an available usage hint.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let area = f.area();
/// codetether_composer_view::render_suggestions(f, &[], 0, area, || Some("usage"));
/// # }
/// ```
pub fn render_suggestions<'a>(
    f: &mut Frame,
    suggestions: &[String],
    selected: usize,
    area: Rect,
    usage_hint: impl FnOnce() -> Option<&'a str>,
) {
    let mut list_state = ListState::default();
    if suggestions.is_empty() {
        if let Some(hint) = usage_hint() {
            crate::suggestion_hint::render(f, area, hint, &mut list_state);
        }
        return;
    }
    let items: Vec<ListItem<'static>> = suggestions
        .iter()
        .enumerate()
        .map(|(idx, cmd)| {
            let prefix = if idx == selected { "▶ " } else { "  " };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(cmd.clone(), Style::default().fg(Color::Cyan).bold()),
            ]))
        })
        .collect();
    list_state.select(Some(selected));
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Commands "))
        .highlight_style(highlight_style());
    f.render_stateful_widget(list, area, &mut list_state);
}
