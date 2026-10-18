//! Snapshot lookup and global window allocation.
use super::*;

impl MuxSnapshot {
    /// Create a server snapshot with one session and its initial window.
    pub fn new(name: String, workspace: PathBuf, isolation: Isolation) -> Self {
        Self {
            sessions: vec![MuxSession::new(name, workspace.clone())],
            workspace,
            isolation,
        }
    }

    /// Find a session by its exact name.
    pub fn session(&self, name: &str) -> Option<&MuxSession> {
        self.sessions.iter().find(|session| session.name == name)
    }

    /// Mutably find a session by its exact name.
    pub fn session_mut(&mut self, name: &str) -> Option<&mut MuxSession> {
        self.sessions
            .iter_mut()
            .find(|session| session.name == name)
    }

    /// Slot for the next window across every session; PTY ids are server-wide.
    pub fn next_window_id(&self) -> u64 {
        self.sessions
            .iter()
            .flat_map(|session| session.windows.iter().map(|window| window.id))
            .max()
            .map_or(0, |id| id + 1)
    }
}
