//! Building session summaries from registry records and live snapshots.

use super::MuxSessionSummary;
use super::summary_window::MuxWindowSummary;
use crate::mux::model::{MuxSession, MuxSnapshot};
use crate::mux::registry::{MuxRecord, SessionTarget};

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

    pub(super) fn from_target(target: &SessionTarget, reachable: bool) -> Self {
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

    fn new(record: &MuxRecord, state: &MuxSnapshot, session: &MuxSession, reachable: bool) -> Self {
        Self {
            name: session.name.clone(),
            workspace: state.workspace.clone(),
            address: record.address.to_string(),
            pid: record.pid,
            active_window: session.active_window,
            windows: session.windows.iter().map(MuxWindowSummary::from).collect(),
            reachable,
            runtime: session.runtime.clone(),
        }
    }
}
