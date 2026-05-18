//! Slash-command autocomplete suggestions list.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::tui::app::state::App;

/// Render the autocomplete suggestions panel in `area`.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::suggestions::render_suggestions;
/// # fn d(f:&mut ratatui::Frame,a:&codetether_agent::tui::app::state::App){ render_suggestions(f,a,ratatui::layout::Rect::new(0,20,80,5)); }
/// ```
pub fn render_suggestions(f: &mut Frame, app: &App, area: Rect) {
    let mut list_state = ListState::default();
    // Empty prefix-match list + a known command → inline usage hint so the
    // user sees `<args>` shape while typing past the command name.
    if app.state.slash_suggestions.is_empty() {
        let Some(hint) = app.state.current_slash_hint() else {
            return;
        };
        let style = Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM);
        let item = ListItem::new(Line::from(vec![
            Span::raw("  "),
            Span::styled(hint.to_string(), style),
        ]));
        let list =
            List::new(vec![item]).block(Block::default().borders(Borders::ALL).title(" Usage "));
        f.render_stateful_widget(list, area, &mut list_state);
        return;
    }
    let items: Vec<ListItem<'static>> = app
        .state
        .slash_suggestions
        .iter()
        .enumerate()
        .map(|(idx, cmd)| {
            let prefix = if idx == app.state.selected_slash_suggestion {
                "▶ "
            } else {
                "  "
            };
            ListItem::new(Line::from(vec![
                Span::raw(prefix),
                Span::styled(cmd.clone(), Style::default().fg(Color::Cyan).bold()),
            ]))
        })
        .collect();
    list_state.select(Some(app.state.selected_slash_suggestion));
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(" Commands "))
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold());
    f.render_stateful_widget(list, area, &mut list_state);
}
