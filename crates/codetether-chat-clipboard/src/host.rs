//! Frontend state and effects for chat clipboard workflows.
use codetether_chat_model::ChatMessage;
use codetether_image::ImageAttachment;
/// Bind chat clipboard operations to frontend state.
/// # Implementors
/// Applications expose their existing messages and pending images and forward
/// normalized text through their usual paste handler.
pub trait Host {
    /// Transcript in chronological order.
    fn messages(&self) -> &[ChatMessage];
    /// Pending image attachments for the next message.
    fn pending_images(&mut self) -> &mut Vec<ImageAttachment>;
    /// Replace frontend status text.
    fn set_status(&mut self, status: String);
    /// Scroll the conversation to the latest content.
    fn scroll_bottom(&mut self);
    /// Insert text through the frontend's chat paste path.
    fn paste_into_chat(&mut self, text: &str);
}
