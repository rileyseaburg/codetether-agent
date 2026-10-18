//! Native preference fields and save operations for shared settings policy.
mod cycles;
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
pub(super) async fn cycle(
    app: &mut App,
    session: &Session,
    setting: codetether_settings_policy::cycles::Cycle,
) {
    codetether_settings_policy::cycles::cycle::<cycles::ProviderRuntime, _, _>(
        &mut view::Settings(app),
        session,
        session.metadata.model.as_deref().unwrap_or_default(),
        setting,
    )
    .await;
}
