//! Session/model label extraction for the chat-panel title.

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::ui::status_bar::session_model_label;

/// The truncated session id label (`new` when no session yet).
///
/// # Examples
///
/// ```rust,no_run
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// use codetether_agent::tui::ui::chat_view::title::labels::session_label;
/// let _s = session_label(app);
/// # }
/// ```
pub fn session_label(app: &App) -> String {
    app.state
        .session_id
        .as_deref()
        .map(|id| {
            if id.len() > 18 {
                format!("{}…", &id[..18])
            } else {
                id.to_string()
            }
        })
        .unwrap_or_else(|| "new".to_string())
}

/// The active model label (`auto` when unresolved).
///
/// # Examples
///
/// ```rust,no_run
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// use codetether_agent::tui::app::session_runtime::SessionView;
/// use codetether_agent::tui::ui::chat_view::title::labels::model_label;
/// let _m = model_label(app, &SessionView::default());
/// # }
/// ```
pub fn model_label(app: &App, session: &SessionView) -> String {
    session
        .model
        .clone()
        .or_else(|| crate::tui::app::spawn_agent::model::current_model_id(app))
        .or_else(|| session_model_label(&app.state))
        .unwrap_or_else(|| "auto".to_string())
}
