//! Native payload specialization of shared startup channels.
use crate::{session::SessionEvent, tui::app::session_runtime::SessionNotice};
pub(super) type SessionChannels =
    codetether_tui_startup::channels::SessionChannels<SessionEvent, SessionNotice>;
pub(super) fn session() -> SessionChannels {
    codetether_tui_startup::channels::session()
}
