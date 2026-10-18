//! Buffer metadata and backend access.
use super::{file_buffer::FileBuffer, helix_backend::HelixBackend};
use std::path::Path;
impl FileBuffer {
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

    /// Mutable backend access that does **not** mark the buffer dirty.
    ///
    /// Used for cursor movement, which is navigation rather than an edit.
    pub(super) fn backend_for_cursor(&mut self) -> &mut HelixBackend {
        &mut self.backend
    }

    /// Recomputes syntax highlights after a content change.
    pub fn refresh_highlight(&mut self) {
        self.backend.rebuild_highlight();
    }
}
