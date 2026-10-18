//! Native preference fields and save operations for shared settings policy.
#[path = "settings_host/access.rs"]
mod access;
#[path = "settings_host/cycles.rs"]
mod cycles;
#[path = "settings_host/session.rs"]
mod session;
use crate::session::Session;
use crate::tui::app::state::App;
pub(in crate::tui::app) use codetether_settings_policy::persistence::Preference;
pub(super) async fn cycle_access(app: &mut App, session: &mut Session) {
    access::cycle(app, session).await;
}
pub(in crate::tui::app) async fn set(
    app: &mut App,
    session: &mut Session,
    preference: Preference,
    next: bool,
) {
    codetether_settings_policy::persistence::set(&mut view(app), session, preference, next).await;
}
pub(super) async fn cycle(
    app: &mut App,
    session: &Session,
    setting: codetether_settings_policy::cycles::Cycle,
) {
    codetether_settings_policy::cycles::cycle::<cycles::ProviderRuntime, _, _>(
        &mut view(app),
        session,
        session.metadata.model.as_deref().unwrap_or_default(),
        setting,
    )
    .await;
}

fn view(app: &mut App) -> codetether_settings_policy::persistence::BorrowedView<'_> {
    codetether_settings_policy::persistence::BorrowedView {
        preferences: &mut app.state.preferences,
        status: &mut app.state.status,
    }
}
