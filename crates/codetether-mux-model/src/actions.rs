//! Window mutations for one mux session.

use std::path::PathBuf;

use anyhow::{Result, bail};

use super::{MuxSession, MuxWindow};

impl MuxSession {
    /// Add a window with a server-allocated id and make it active.
    pub fn create_window(&mut self, id: u64, workspace: PathBuf) {
        self.windows.push(MuxWindow::new(id, workspace));
        self.active_window = id;
    }

    /// Select an existing window.
    /// # Errors
    /// Returns an error when `id` is absent from the session.
    pub fn select_window(&mut self, id: u64) -> Result<()> {
        self.windows
            .iter()
            .any(|window| window.id == id)
            .then(|| self.active_window = id)
            .ok_or_else(|| anyhow::anyhow!("window {id} does not exist"))
    }

    /// Remove a window and select the first remaining window when necessary.
    /// # Errors
    /// Returns an error for the last window or an unknown ID.
    /// # Panics
    /// Malformed state with duplicate IDs can leave no replacement window.
    pub fn close_window(&mut self, id: u64) -> Result<()> {
        if self.windows.len() == 1 {
            bail!("cannot close the last window");
        }
        let before = self.windows.len();
        self.windows.retain(|window| window.id != id);
        if self.windows.len() == before {
            bail!("window {id} does not exist");
        }
        if self.active_window == id {
            self.active_window = self.windows[0].id;
        }
        Ok(())
    }

    /// Replace the active window's working directory and derived title.
    pub fn change_directory(&mut self, workspace: PathBuf) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.id == self.active_window)
        {
            *window = MuxWindow::new(window.id, workspace);
        }
    }
}
