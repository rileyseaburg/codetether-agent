//! Window mutations for a mux session.

use std::path::PathBuf;

use anyhow::{Result, bail};

use super::{MuxSnapshot, MuxWindow};

impl MuxSnapshot {
    pub(in crate::mux) fn create_window(&mut self, workspace: PathBuf) {
        let id = self
            .windows
            .iter()
            .map(|window| window.id)
            .max()
            .unwrap_or(0)
            + 1;
        self.windows.push(MuxWindow::new(id, workspace));
        self.active_window = id;
    }

    pub(in crate::mux) fn select_window(&mut self, id: u64) -> Result<()> {
        self.windows
            .iter()
            .any(|window| window.id == id)
            .then(|| self.active_window = id)
            .ok_or_else(|| anyhow::anyhow!("window {id} does not exist"))
    }

    pub(in crate::mux) fn close_window(&mut self, id: u64) -> Result<()> {
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

    pub(in crate::mux) fn change_directory(&mut self, workspace: PathBuf) {
        if let Some(window) = self
            .windows
            .iter_mut()
            .find(|window| window.id == self.active_window)
        {
            *window = MuxWindow::new(window.id, workspace);
        }
    }
}
