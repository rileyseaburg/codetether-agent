//! Forward objective approval keys through native state bindings.
use crate::tui::app::state::App;
use crossterm::event::KeyEvent;
#[cfg(test)]
#[path = "okr_key_tests.rs"]
mod tests;
#[cfg(test)]
pub(super) use codetether_relay_okr::interactive::approval::is_action;
pub async fn handle(app: &mut App, key: KeyEvent) -> bool {
    codetether_relay_okr::interactive::approval::handle(&mut super::host::Native(app), key).await
}
