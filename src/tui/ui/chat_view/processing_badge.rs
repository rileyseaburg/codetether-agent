//! Application-value adapter for the shared processing_badge status renderer.
use crate::tui::app::state::App;
use ratatui::text::Span;
/// Render the status component from current application values.
///
/// # Arguments
/// * `app` - Application values used for this status component.
///
/// # Returns
/// The component's owned spans, or no badge when inactive.
///
/// # Examples
/// ```no_run
/// # fn example(app: &codetether_agent::tui::app::state::App) {
/// let _display = codetether_agent::tui::ui::chat_view::processing_badge::processing_badge(app);
/// # }
/// ```
pub fn processing_badge(app: &App) -> Option<Span<'static>> {
    codetether_chat_status::processing_badge::processing_badge(app.state.processing, || {
        app.state.worker_task_queue.len()
            + usize::from(app.state.active_remote_task.is_some())
            + app.state.active_tasks.count()
    })
}
