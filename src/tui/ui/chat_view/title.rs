//! Chat-panel block title with neon gradient styling.
//!
//! [`build_title`] returns a [`Line`] with colored spans for the messages-panel
//! title bar: a gradient `⬡ CodeTether` brand, then dim model/session labels.

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;

use super::spinner::spinner_color;

#[path = "title_brand.rs"]
pub mod brand;
#[path = "title_labels.rs"]
pub mod labels;

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
    let neon = spinner_color();
    let mut spans = vec![Span::styled(
        " ⬡ ",
        Style::default().fg(neon).add_modifier(Modifier::BOLD),
    )];
    spans.extend(brand::brand_spans());
    spans.extend([
        Span::styled(" ▸ ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("model:{}", labels::model_label(app, session)),
            Style::default().fg(Color::Cyan),
        ),
        Span::styled("  session:", Style::default().fg(Color::DarkGray)),
        Span::styled(
            labels::session_label(app),
            Style::default().fg(Color::Yellow),
        ),
        Span::raw(" "),
    ]);
    Line::from(spans)
}
