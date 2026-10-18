//! One session on one server: the unit a client connects to.

use codetether_mux_model::MuxSession;

use super::MuxRecord;

#[derive(Clone, Debug)]
/// A named session resolved to its hosting server record.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let target = codetether_mux_registry::load("work").await?;
/// assert_eq!(target.session, "work");
/// # Ok(()) }
/// ```
pub struct SessionTarget {
    /// Hosting server's discovery record.
    pub record: MuxRecord,
    /// Session selected within that server.
    pub session: String,
}

impl SessionTarget {
    /// Find the selected session in the persisted snapshot.
    pub fn session(&self) -> Option<&MuxSession> {
        self.record.state.session(&self.session)
    }

    /// Id of the session's active window, which hosts its TUI.
    /// # Errors
    /// Returns an error when the selected session is absent from the record.
    pub fn active_window(&self) -> anyhow::Result<u64> {
        self.session()
            .map(|session| session.active_window)
            .ok_or_else(|| anyhow::anyhow!("mux session '{}' is unavailable", self.session))
    }
}
