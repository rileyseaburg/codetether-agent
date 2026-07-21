//! Session/model label extraction for the chat-panel title.

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::ui::status_bar::session_model_label;

/// The truncated session title or id (`new` when no session exists).
///
/// # Examples
///
/// ```rust,no_run
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// use codetether_agent::tui::ui::chat_view::title::labels::session_label;
/// let _label = session_label(app);
/// # }
/// ```
pub fn session_label(app: &App) -> String {
    let Some(id) = app.state.session_id.as_deref() else {
        return "new".to_string();
    };
    let label = app
        .state
        .sessions
        .get(app.state.selected_session)
        .filter(|session| session.id == id)
        .map(crate::session::SessionSummary::display_label)
        .unwrap_or_else(|| id.to_string());
    compact(&label)
}

fn compact(label: &str) -> String {
    if label.len() > 28 {
        format!("{}…", crate::util::truncate_bytes_safe(label, 27))
    } else {
        label.to_string()
    }
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
