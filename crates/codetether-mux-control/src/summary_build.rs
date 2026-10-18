//! Building session summaries from registry records and live snapshots.

use super::MuxSessionSummary;
use codetether_mux_model::{MuxSession, MuxSnapshot};
use codetether_mux_registry::{MuxRecord, SessionTarget};

impl MuxSessionSummary {
    /// Every session hosted by one server record.
    pub(super) fn from_record(record: &MuxRecord, reachable: bool) -> Vec<Self> {
        record
            .state
            .sessions
            .iter()
            .map(|session| Self::new(record, &record.state, session, reachable))
            .collect()
    }

    /// Project a discovered target into a session summary.
    ///
    /// # Arguments
    /// * `target` - Discovered server record and selected session.
    /// * `reachable` - Whether the caller successfully contacted the server.
    /// # Returns
    /// A summary marked unreachable if its session is missing from the record.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let target = codetether_mux_registry::load("work").await?;
    /// let summary = codetether_mux_control::MuxSessionSummary::from_target(&target, false);
    /// assert!(!summary.reachable);
    /// # Ok(()) }
    /// ```
    pub fn from_target(target: &SessionTarget, reachable: bool) -> Self {
        Self::from_state(
            &target.record,
            &target.record.state,
            &target.session,
            reachable,
        )
    }

    /// Summary of `name` within a fresh `state`; unknown names report unreachable.
    pub(super) fn from_state(
        record: &MuxRecord,
        state: &MuxSnapshot,
        name: &str,
        reachable: bool,
    ) -> Self {
        let placeholder = MuxSession::new(name.into(), state.workspace.clone());
        match state.session(name) {
            Some(session) => Self::new(record, state, session, reachable),
            None => Self::new(record, state, &placeholder, false),
        }
    }
}
