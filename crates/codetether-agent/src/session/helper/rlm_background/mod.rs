//! Native background services specialized to application session events.
pub(super) use codetether_rlm_background::resolve_pending;
pub(crate) type Background = codetether_native_proactive::Background<crate::session::Session>;
pub(crate) type Notify = codetether_native_proactive::Notify<crate::session::Session>;
