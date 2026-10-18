//! Frontend views and effects used by chat file actions.
use codetether_chat_layout::ChatHit;
use codetether_chat_model::ChatMessage;
use std::path::Path;
/// Frontend adapter for file-reference actions.
/// # Implementors
/// Applications expose transcript/hit-test state and existing file-opening UI.
pub trait Host {
    /// Whether chat shortcuts and clicks are currently active.
    fn in_chat(&self) -> bool;
    /// Current ordered transcript.
    fn messages(&self) -> &[ChatMessage];
    /// Geometry and visible text from the last chat render.
    fn chat_hit(&self) -> &ChatHit;
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
    /// Replace the captured Git diff.
    fn set_diff(&mut self, diff: String);
    /// Record when the Git diff was captured.
    fn set_captured_at(&mut self, time: String);
    /// Preserve current scroll state before changing views.
    fn save_scroll_state(&mut self);
    /// Switch to the Git diff view.
    fn show_git(&mut self);
    /// Open a file in the frontend viewer.
    fn open_viewer(&mut self, cwd: &Path, file: &Path);
    /// Open a file in the frontend editor.
    fn open_editor(&mut self, cwd: &Path, file: &Path);
}
