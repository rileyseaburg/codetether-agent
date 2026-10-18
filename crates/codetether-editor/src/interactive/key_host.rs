//! Frontend bindings needed for editor action dispatch.
use crate::{EditorInput, FileBuffer};
use std::path::Path;
/// Frontend state and integrations used by synchronous editor keys.
/// # Implementors
/// UIs provide their buffer, mode and approval integration through this trait.
pub trait KeyHost {
    /// Whether the current view is the editor.
    fn in_editor(&self) -> bool;
    /// Whether an approval proposal is currently being edited.
    fn approval_editing(&self) -> bool;
    /// Offer an action to the approval editor and report whether it handled it.
    fn handle_approval_action(&mut self, action: &EditorInput) -> bool;
    /// Borrow the current editor buffer slot.
    fn editor(&mut self) -> &mut Option<FileBuffer>;
    /// Select the chat view after an editor exit.
    fn show_chat(&mut self);
    /// Reset both editor scroll offsets.
    fn reset_scroll(&mut self);
    /// Replace the frontend status text.
    fn set_status(&mut self, status: String);
    /// Request a redraw.
    fn redraw(&mut self);
    /// Open the workspace file finder.
    fn open_finder(&mut self, cwd: &Path);
}
