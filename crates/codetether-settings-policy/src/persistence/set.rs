//! Preference synchronization, environment policy and durable save ordering.
use super::{Preference, Session, View, save};
/// Update interactive and stored preferences, then persist and report status.
/// # Arguments
/// * `view` - Interactive preference values and status output.
/// * `session` - Existing session metadata and persistence operations.
/// * `preference` - Preference to update.
/// * `next` - New boolean value.
/// # Returns
/// Completes after saving. Network changes also update the existing sandbox override.
/// # Examples
/// ```no_run
/// # async fn example<V: codetether_settings_policy::persistence::View, S: codetether_settings_policy::persistence::Session>(view: &mut V, session: &mut S) {
/// codetether_settings_policy::persistence::set(view, session, codetether_settings_policy::persistence::Preference::Autocomplete, true).await;
/// # }
/// ```
pub async fn set<V: View, S: Session>(
    view: &mut V,
    session: &mut S,
    preference: Preference,
    next: bool,
) {
    view.set_preference(preference, next);
    session.set_preference(preference, next);
    if let Preference::Network = preference {
        if next {
            unsafe { std::env::set_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK", "1") }
        } else {
            unsafe { std::env::remove_var("CODETETHER_SANDBOX_BASH_ALLOW_NETWORK") }
        }
    }
    save(view, session, preference.status(next)).await;
}
