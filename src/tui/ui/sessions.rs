//! Application projection into the shared session picker.
use crate::tui::app::state::App;
use ratatui::Frame;
pub(super) mod window {
    pub(crate) use codetether_session_picker::bounds;
}
/// Render sessions from the application and supply its live bus status.
///
/// # Arguments
/// * `f` - Destination terminal frame.
/// * `app` - Application containing session metadata and status.
///
/// # Returns
/// Draws the session picker into the frame.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame, app: &mut codetether_agent::tui::app::state::App) {
/// codetether_agent::tui::ui::sessions::render_sessions_view(f, app);
/// # }
/// ```
pub fn render_sessions_view(f: &mut Frame, app: &mut App) {
    let view = codetether_session_picker::PickerView {
        sessions: &app.state.sessions,
        filter: &app.state.session_filter,
        selected: app.state.selected_session,
        active_session: app.state.session_id.as_deref(),
        status: &app.state.status,
    };
    codetether_session_picker::render_sessions_view(f, &view, || {
        crate::tui::ui::status_bar::bus_status_badge_span(app)
    });
}
