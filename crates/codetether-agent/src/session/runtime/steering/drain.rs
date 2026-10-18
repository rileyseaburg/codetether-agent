//! Native session specialization of shared steering drains.
use crate::session::Session;
pub(crate) fn drain_into(session: &mut Session) -> usize {
    codetether_session_steering::drain_into(session)
}
#[cfg(test)]
pub(crate) fn drain_or_close_into(session: &mut Session) -> usize {
    codetether_session_steering::drain_or_close_into(session)
}
