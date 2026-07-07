//! Chat-panel block title with neon gradient styling.
//!
//! [`build_title`] returns a [`Line`] with colored spans for the messages-panel
//! title bar: a neon `⬡ CodeTether` brand, then dim model/session labels.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::ui::status_bar::session_model_label;

use super::spinner::spinner_color;

/// Build the messages-panel neon title [`Line`].
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::title::build_title;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let view = codetether_agent::tui::app::session_runtime::SessionView::default();
/// let _line = build_title(app, &view);
/// # }
/// ```
pub fn build_title(app: &App, session: &SessionView) -> Line<'static> {
    let session_label = app
        .state
        .session_id
        .as_deref()
        .map(|id| {
            if id.len() > 18 {
                format!("{}…", &id[..18])
            } else {
                id.to_string()
            }
        })
        .unwrap_or_else(|| "new".to_string());
    let model_label = session
        .model
        .clone()
        .or_else(|| session_model_label(&app.state))
        .unwrap_or_else(|| "auto".to_string());
    let neon = spinner_color();
    Line::from(vec![
        Span::styled(
            " ⬡ ",
            Style::default().fg(neon).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            "CodeTether",
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" ▸ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("model:{model_label}"),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("  session:", Style::default().fg(Color::DarkGray)),
        Span::styled(session_label, Style::default().fg(Color::Yellow)),
        Span::raw(" "),
    ])
}
