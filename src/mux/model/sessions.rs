//! Session lifecycle mutations for one workspace mux server.

use std::path::PathBuf;

use anyhow::{Result, bail};

use super::{MuxSession, MuxSnapshot};

impl MuxSnapshot {
    /// Create an isolated session rooted at `workspace`, returning its first window id.
    pub(in crate::mux) fn create_session(
        &mut self,
        name: String,
        workspace: PathBuf,
    ) -> Result<u64> {
        if self.session(&name).is_some() {
            bail!("mux session '{name}' already exists");
        }
        let id = self.next_window_id();
        let mut session = MuxSession::new(name, workspace);
        session.windows[0].id = id;
        session.active_window = id;
        self.sessions.push(session);
        Ok(id)
    }

    /// Remove a session, returning its window ids so their PTYs can be stopped.
    pub(in crate::mux) fn close_session(&mut self, name: &str) -> Result<Vec<u64>> {
        let Some(index) = self.sessions.iter().position(|item| item.name == name) else {
            bail!("mux session '{name}' does not exist");
        };
        let removed = self.sessions.remove(index);
        Ok(removed.windows.iter().map(|window| window.id).collect())
    }
}
