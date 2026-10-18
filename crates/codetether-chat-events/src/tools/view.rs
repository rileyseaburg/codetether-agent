//! Native fields and approval delivery used by tool presentation.
use super::Timing;
use codetether_approval_core::interaction::LiveApprovalRequest;
use codetether_chat_model::ChatMessage;
/// Frontend state and effects for tool-call events.
/// # Implementors
/// Frontends retain approval, timing and message storage ownership.
pub trait View: Timing {
    /// Borrow accumulated stream text, moved into chat at tool start.
    fn streaming_text(&mut self) -> &mut String;
    /// Borrow chronological chat messages.
    fn messages(&mut self) -> &mut Vec<ChatMessage>;
    /// Reset the tool preview viewport.
    fn reset_tool_preview_scroll(&mut self);
    /// Record the active tool name.
    fn start_pending_tool(&mut self, name: String);
    /// Record tool completion metrics.
    fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool);
    /// Replace status text.
    fn set_status(&mut self, status: String);
    /// Scroll chat to the newest content.
    fn scroll_to_bottom(&mut self);
    /// Deliver an approval request through the frontend's existing handler.
    fn approval_request(&mut self, request: LiveApprovalRequest);
    /// Current trust-policy guidance, requested only for approval errors.
    fn approval_guidance(&self) -> String;
}
