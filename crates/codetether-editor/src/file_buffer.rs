//! File-backed editor document: load from a path, edit, save.
//!
//! [`FileBuffer`] pairs a [`HelixBackend`] with its on-disk path and a dirty
//! flag, so the TUI can open a file, mutate it through [`EditorEdit`], and
//! write it back. It owns no rendering or input concerns (SRP).

use std::path::{Path, PathBuf};

use super::edit::EditorEdit;
use super::helix_backend::HelixBackend;

/// A rope-backed document tied to a filesystem path.
///
/// # Examples
/// ```
/// let buffer = codetether_editor::FileBuffer::draft("scratch", "hello");
/// assert_eq!(buffer.text(), "hello");
/// assert!(!buffer.is_dirty());
/// ```
#[derive(Debug, Clone)]
pub struct FileBuffer {
    pub(super) path: PathBuf,
    pub(super) backend: HelixBackend,
    pub(super) dirty: bool,
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
}
