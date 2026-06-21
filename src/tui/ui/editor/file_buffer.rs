//! File-backed editor document: load from a path, edit, save.
//!
//! [`FileBuffer`] pairs a [`HelixBackend`] with its on-disk path and a dirty
//! flag, so the TUI can open a file, mutate it through [`EditorEdit`], and
//! write it back. It owns no rendering or input concerns (SRP).

use std::path::{Path, PathBuf};

use super::edit::EditorEdit;
use super::helix_backend::HelixBackend;

/// A rope-backed document tied to a filesystem path.
#[derive(Debug, Clone)]
pub struct FileBuffer {
    path: PathBuf,
    backend: HelixBackend,
    dirty: bool,
}

impl FileBuffer {
    /// Opens `path`, reading its contents (empty if the file is absent).
    ///
    /// # Errors
    ///
    /// Returns an error if the path exists but cannot be read.
    pub fn open(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let path = path.as_ref().to_path_buf();
        let text = match std::fs::read_to_string(&path) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(e),
        };
        Ok(Self {
            path,
            backend: HelixBackend::from_str(&text),
            dirty: false,
        })
    }

    /// Writes the current contents back to the file, clearing the dirty flag.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be written.
    pub fn save(&mut self) -> std::io::Result<()> {
        std::fs::write(&self.path, self.backend.to_text())?;
        self.dirty = false;
        Ok(())
    }

    /// Whether the buffer has unsaved changes.
    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// The file path this buffer is bound to.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Shared access to the render backend.
    pub fn backend(&self) -> &HelixBackend {
        &self.backend
    }

    /// Mutable access that marks the buffer dirty.
    pub fn backend_mut(&mut self) -> &mut HelixBackend {
        self.dirty = true;
        &mut self.backend
    }

    /// Recomputes syntax highlights after a content change.
    pub fn refresh_highlight(&mut self) {
        self.backend.rebuild_highlight();
    }
}
