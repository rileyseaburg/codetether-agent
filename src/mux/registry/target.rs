//! One session on one server: the unit a client connects to.

use crate::mux::model::{MuxSession, MuxWindow};

use super::MuxRecord;

#[derive(Clone, Debug)]
pub(in crate::mux) struct SessionTarget {
    pub record: MuxRecord,
    pub session: String,
}

impl SessionTarget {
    pub(in crate::mux) fn session(&self) -> Option<&MuxSession> {
        self.record.state.session(&self.session)
    }

    /// Id of the session's active window, which hosts its TUI.
    pub(in crate::mux) fn active_window(&self) -> anyhow::Result<u64> {
        self.session()
            .map(|session| session.active_window)
            .ok_or_else(|| anyhow::anyhow!("mux session '{}' is unavailable", self.session))
    }

    pub(in crate::mux) fn active(&self) -> Option<&MuxWindow> {
        self.session().and_then(MuxSession::active)
    }

    pub(in crate::mux) fn runtime(&self) -> Option<&crate::mux::model::MuxRuntimeStatus> {
        self.session().and_then(|session| session.runtime.as_ref())
    }
}
