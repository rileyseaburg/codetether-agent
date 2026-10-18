//! Frontend bindings for pending approval actions.
use crate::input::Preview;
use codetether_approval_edit::ApprovalEditSession;
use codetether_editor::FileBuffer;
/// Frontend fields and command execution used by approval actions.
/// # Implementors
/// UIs lend preview/editor state and preserve their command and view lifecycle.
pub trait Host {
    /// Borrow the preview input and scroll state.
    fn preview(&mut self) -> Preview<'_>;
    /// Run an approval or denial command through the application dispatcher.
    fn run_command(&mut self, command: &str) -> bool;
    /// Borrow the active editor buffer slot.
    fn editor(&mut self) -> &mut Option<FileBuffer>;
    /// Borrow the approval edit session slot.
    fn session(&mut self) -> &mut Option<ApprovalEditSession>;
    /// Reset both editor scroll offsets.
    fn reset_editor_scroll(&mut self);
    /// Enter the editor with the frontend's normal view transition.
    fn show_editor(&mut self);
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
}
