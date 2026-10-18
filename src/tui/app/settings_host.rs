//! Native preference fields and save operations for shared settings policy.
mod session;
mod view;
use crate::session::Session;
use crate::tui::app::state::App;
pub(super) use codetether_settings_policy::persistence::Preference;
pub(super) async fn set(app: &mut App, session: &mut Session, preference: Preference, next: bool) {
    codetether_settings_policy::persistence::set(
        &mut view::Settings(app),
        session,
        preference,
        next,
    )
    .await;
}
pub(super) async fn persist(app: &mut App, session: &Session, message: String) {
    codetether_settings_policy::persistence::save(&mut view::Settings(app), session, message).await;
}
