//! Journal, editor-state and active-session boundaries.
use crate::Draft;
use codetether_editor::FileBuffer;
use codetether_goal_core::{GoalEdit, TaskState};
use std::future::Future;
/// Existing human-only journal operations.
/// # Implementors
/// Bind to native goal reads and authenticated/user update operations.
pub trait Journal {
    /// Read the current goal state.
    /// # Errors
    /// Returns journal read errors.
    fn read(id: &str) -> anyhow::Result<TaskState>;
    /// Persist a human goal edit.
    /// # Errors
    /// Returns validation, conflict or storage errors.
    fn update(id: &str, edit: GoalEdit) -> impl Future<Output = anyhow::Result<()>>;
}
/// Read-only state needed by virtual goal editing.
/// # Implementors
/// Return the existing UI buffer and original goal snapshot.
pub trait Read {
    /// Native journal binding.
    type Journal: Journal;
    /// Current goal draft.
    fn draft(&self) -> Option<&Draft>;
    /// Current editor buffer, including non-goal buffers.
    fn buffer(&self) -> Option<&FileBuffer>;
    /// Whether an approval editor is active.
    fn approval_open(&self) -> bool;
}
/// Mutable UI operations in their existing application order.
/// # Implementors
/// Map each method directly onto native state and view updates.
pub trait Write {
    /// Current mutable editor buffer.
    fn buffer_mut(&mut self) -> Option<&mut FileBuffer>;
    /// Install or remove the virtual buffer.
    fn set_buffer(&mut self, buffer: Option<FileBuffer>);
    /// Install or remove the goal snapshot.
    fn set_draft(&mut self, draft: Option<Draft>);
    /// Reset both editor scroll axes.
    fn reset_scroll(&mut self);
    /// Clear stale editor hover state.
    fn clear_hover(&mut self);
    /// Clear chat input when opening a goal editor.
    fn clear_input(&mut self);
    /// Switch to the editor view.
    fn show_editor(&mut self);
    /// Switch back to chat.
    fn show_chat(&mut self);
    /// Replace status text.
    fn status(&mut self, text: String);
    /// Request redraw.
    fn redraw(&mut self);
}
/// Complete editor state interface.
/// # Implementors
/// All readable and writable editor states implement this trait.
pub trait Host: Read + Write {}
impl<T: Read + Write> Host for T {}
/// Session-aware key/save context wrapping an editor state.
/// # Implementors
/// Preserve native session-view identity and deferred restart behavior.
pub trait Session {
    /// Editor state bound to this interaction.
    type Editor: Host;
    /// Borrow editor state.
    fn editor(&self) -> &Self::Editor;
    /// Mutably borrow editor state.
    fn editor_mut(&mut self) -> &mut Self::Editor;
    /// Current session identity.
    fn session_id(&self) -> &str;
    /// Schedule continuation after a saved human edit.
    fn restart(&mut self, id: &str);
}
