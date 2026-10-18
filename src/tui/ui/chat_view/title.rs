//! Application adapter for the shared chat title.
use crate::tui::app::{session_runtime::SessionView, state::App};
use ratatui::text::Line;
#[path = "title_labels.rs"]
pub mod labels;
pub use codetether_chat_status::title_brand as brand;
/// Build the chat title, resolving labels at their original rendering points.
///
/// # Arguments
/// * `app` - Application providing the current model and session label.
/// * `session` - Active session view.
///
/// # Returns
/// Owned title spans from the shared renderer.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App, session: &codetether_agent::tui::app::session_runtime::SessionView) {
/// let _title = codetether_agent::tui::ui::chat_view::title::build_title(app, session);
/// # }
/// ```
pub fn build_title(app: &App, session: &SessionView) -> Line<'static> {
    codetether_chat_status::build_title(
        || labels::model_label(app, session),
        || labels::session_label(app),
    )
}
