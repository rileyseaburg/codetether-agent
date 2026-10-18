use crate::tui::app::state::App;
use crate::tui::models::ViewMode;

mod new_session;
mod picker_host;
mod upsert;

pub use new_session::create as create_new_session;
pub use upsert::upsert_active_session;
/// Refresh workspace sessions through shared picker policy.
pub async fn refresh_sessions(app: &mut App, cwd: &std::path::Path) {
    codetether_session_picker::refresh::refresh::<picker_host::Catalog, _>(
        &mut picker_host::Picker(app),
        cwd,
    )
    .await;
}

pub fn return_to_chat(app: &mut App) {
    app.state.restore_scroll_state();
    app.state.set_view_mode(ViewMode::Chat);
    app.state.status = "Back to chat".to_string();
}
