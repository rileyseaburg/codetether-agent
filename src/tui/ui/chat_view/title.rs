//! Chat-panel block title (model + session labels).
//!
//! [`build_title`] composes the title bar string shown above the messages
//! area, e.g. `CodeTether Agent [main] model:gpt-4o session:abc-123…`.

use crate::session::Session;
use crate::tui::app::state::App;
use crate::tui::ui::status_bar::session_model_label;

/// Build the messages-panel title string from the active model and session.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::title::build_title;
/// # fn demo(app: &codetether_agent::tui::app::state::App, sess: &codetether_agent::session::Session) {
/// let title = build_title(app, sess);
/// assert!(title.contains("model:"));
/// # }
/// ```
pub fn build_title(app: &App, session: &Session) -> String {
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
        .metadata
        .model
        .clone()
        .or_else(|| session_model_label(&app.state))
        .unwrap_or_else(|| "auto".to_string());
    format!(" CodeTether Agent [main] model:{model_label} session:{session_label} ")
}
