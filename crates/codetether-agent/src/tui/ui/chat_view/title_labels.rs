//! Session/model label extraction for the chat-panel title.

use crate::tui::app::session_runtime::SessionView;
use crate::tui::app::state::App;
use crate::tui::ui::status_bar::session_model_label;
/// Resolve the active session display label through its shared owner.
///
/// # Arguments
/// * `app` - Application supplying the active session and picker selection.
///
/// # Returns
/// A compact title or identifier, or new when no session exists.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _label = codetether_agent::tui::ui::chat_view::title::labels::session_label(app);
/// # }
/// ```
pub fn session_label(app: &App) -> String {
    codetether_session_picker::session_label(
        &app.state.session_picker.sessions,
        app.state.session_picker.selected_session,
        app.state.session_id.as_deref(),
    )
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
