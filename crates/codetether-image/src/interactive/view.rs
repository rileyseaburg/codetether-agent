//! Pending-image queue and command output contracts.
use crate::ImageAttachment;
/// Output behavior for the existing image-command contexts.
/// # Examples
/// ```
/// let mode = codetether_image::interactive::Presentation::Chat;
/// ```
#[derive(Clone, Copy)]
pub enum Presentation {
    /// Full chat status plus a system notice.
    Chat,
    /// Pending attachment status without a system notice.
    Pending,
}
/// Interactive client operations required to attach an image.
/// # Implementors
/// Native clients supply their current queue and output functions.
pub trait View {
    /// Append the attachment and return the resulting queue length.
    fn push_image(&mut self, image: ImageAttachment) -> usize;
    /// Replace the interactive status message.
    fn set_status(&mut self, message: String);
    /// Append a system notice using the client's normal scrolling behavior.
    fn push_system_message(&mut self, message: String);
}
