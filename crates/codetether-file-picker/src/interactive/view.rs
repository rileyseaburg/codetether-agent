//! Frontend state and attachment integrations for the file picker.
use crate::FilePickerState;
use codetether_image::ImageAttachment;
use std::path::Path;
/// State and effects used when opening or attaching a picked file.
/// # Implementors
/// Frontends retain their composer, image queue and view selection.
pub trait View {
    /// Borrow the current browser/viewer state.
    fn picker(&mut self) -> &mut FilePickerState;
    /// Select the file-picker view.
    fn show_picker(&mut self);
    /// Select chat after an attachment attempt.
    fn show_chat(&mut self);
    /// Replace status text.
    fn set_status(&mut self, status: String);
    /// Append an image attachment.
    fn push_image(&mut self, image: ImageAttachment);
    /// Add a system notice to the transcript.
    fn push_system(&mut self, text: String);
    /// Add an error notice to the transcript.
    fn push_error(&mut self, text: String);
    /// Scroll chat to its latest content.
    fn scroll_to_bottom(&mut self);
    /// Share a text file through the frontend's existing composer flow.
    fn attach_file(&mut self, workspace: &Path, path: &Path);
}
