//! Virtual editor buffers used to revise approval proposals before execution.

use std::path::Path;

use super::{edit::EditorEdit, file_buffer::FileBuffer, helix_backend::HelixBackend};

impl FileBuffer {
    /// Creates a dirty file-bound buffer from proposed in-memory content.
    ///
    /// Unlike [`FileBuffer::open`], this does not read or write `path`. Saving
    /// remains controlled by the approval editor, which converts the content
    /// back into a revised patch rather than bypassing the approval gate.
    pub(crate) fn proposed(path: impl AsRef<Path>, text: &str) -> Self {
        Self {
            path: path.as_ref().to_path_buf(),
            backend: HelixBackend::from_str(text),
            dirty: true,
        }
    }

    /// Returns the complete current document text.
    pub(crate) fn text(&self) -> String {
        self.backend.to_text()
    }
}
