//! Editor buffers, revision storage and patch export.
use crate::{ApprovalEditSession, load, patch};
use codetether_editor::FileBuffer;
use std::path::Path;
impl ApprovalEditSession {
    /// Load proposed file contents and initialize an editing session.
    /// # Arguments
    /// * `root` - Workspace root for patch path resolution.
    /// * `id` - Pending approval ID.
    /// * `patch` - Proposed patch text.
    /// # Returns
    /// A session positioned on the first proposed file.
    /// # Errors
    /// Returns patch parsing/path errors or an error when no files are editable.
    /// # Examples
    /// ```no_run
    /// let _ = codetether_approval_edit::ApprovalEditSession::from_patch(std::path::Path::new("."), "id".into(), "patch text");
    /// ```
    pub fn from_patch(root: &Path, id: String, patch: &str) -> anyhow::Result<Self> {
        load::session(root, id, patch)
    }

    /// Build an in-memory editor buffer for the current proposed file.
    /// # Examples
    /// ```no_run
    /// # fn example(session: &codetether_approval_edit::ApprovalEditSession) {
    /// let _ = session.buffer();
    /// # }
    /// ```
    pub fn buffer(&self) -> FileBuffer {
        let file = &self.files[self.index];
        FileBuffer::proposed(&file.path, &file.revised)
    }

    /// Store the current file revision and advance when another file remains.
    /// # Arguments
    /// * `text` - Revised contents from the editor.
    /// # Returns
    /// True if another file is ready for editing.
    /// # Examples
    /// ```no_run
    /// # fn example(session: &mut codetether_approval_edit::ApprovalEditSession) {
    /// let _ = session.store("revised".into());
    /// # }
    /// ```
    pub fn store(&mut self, text: String) -> bool {
        self.files[self.index].revised = text;
        let more = self.index + 1 < self.files.len();
        if more {
            self.index += 1;
        }
        more
    }

    /// Return the one-based file position and total file count.
    pub fn progress(&self) -> (usize, usize) {
        (self.index + 1, self.files.len())
    }

    /// Build a unified patch containing only changed file contents.
    pub fn revised_patch(&self) -> String {
        patch::build(&self.files)
    }
}
