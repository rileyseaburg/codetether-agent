//! Concrete native session slot with a TUI metadata snapshot.
use crate::session::Session;
pub(crate) type SessionSlot = codetether_session_runtime::slot::SessionSlot<Session>;
codetether_session_runtime::impl_frontend_session_view!(
    Session,
    crate::tui::app::session_runtime::SessionView,
    crate::provenance::RuntimePrincipal,
);
