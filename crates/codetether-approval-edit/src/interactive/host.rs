//! UI state required by approval-backed editor actions.
use crate::ApprovalEditSession;
use codetether_editor::FileBuffer;
/// Frontend bindings for approval editor state and view transitions.
/// # Implementors
/// Frontends expose their existing session and buffer slots without copying.
pub trait Host {
    /// Access the current editor buffer.
    fn editor(&mut self) -> &mut Option<FileBuffer>;
    /// Access the current proposed patch editing session.
    fn session(&mut self) -> &mut Option<ApprovalEditSession>;
    /// Reset both editor viewport offsets.
    fn reset_scroll(&mut self);
    /// Return to chat using the frontend's normal view transition.
    fn show_chat(&mut self);
    /// Reset approval preview scrolling.
    fn reset_preview_scroll(&mut self);
    /// Record whether an approval remains active.
    fn set_waiting(&mut self, waiting: bool);
    /// Replace the frontend status message.
    fn set_status(&mut self, status: String);
    /// Request frontend redraw.
    fn redraw(&mut self);
}
