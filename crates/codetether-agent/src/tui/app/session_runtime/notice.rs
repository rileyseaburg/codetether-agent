//! Completion notices specialized for native session ownership.
pub(crate) type SessionNotice =
    codetether_session_events::completion::SessionNotice<crate::session::Session>;
