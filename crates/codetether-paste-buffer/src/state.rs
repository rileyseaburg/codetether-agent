//! Pending image attachments and text sidecars owned by one draft message.
use crate::PendingTextPaste;
use codetether_image::ImageAttachment;

/// Attachment buffers retained until the current draft is submitted or steered.
///
/// # Examples
/// ```
/// let mut state = codetether_paste_buffer::AttachmentState::default();
/// let placeholder = state.attach_text("hello".into());
/// assert!(placeholder.contains("#1"));
/// assert_eq!(state.take_text().len(), 1);
/// assert!(state.text.is_empty());
/// ```
#[derive(Debug, Default)]
pub struct AttachmentState {
    /// Images pending submission with the current draft.
    pub images: Vec<ImageAttachment>,
    /// Full text represented by placeholders in the draft input.
    pub text: Vec<PendingTextPaste>,
}

impl AttachmentState {
    /// Store a text sidecar and return its input placeholder.
    /// # Arguments
    /// * `content` — Original text to retain until submission.
    /// # Returns
    /// The placeholder for the newly stored text.
    /// # Panics
    /// May panic if incrementing the largest existing identifier overflows.
    /// # Examples
    /// See [`AttachmentState`] for text attachment and consumption.
    pub fn attach_text(&mut self, content: String) -> String {
        crate::attach_paste(&mut self.text, content)
    }

    /// Take the image buffer, leaving it empty for the next draft.
    /// # Returns
    /// All pending images in insertion order.
    /// # Examples
    /// ```
    /// let mut state = codetether_paste_buffer::AttachmentState::default();
    /// assert!(state.take_images().is_empty());
    /// ```
    pub fn take_images(&mut self) -> Vec<ImageAttachment> {
        std::mem::take(&mut self.images)
    }

    /// Take the text buffer, leaving it empty for the next draft.
    /// # Returns
    /// All pending text sidecars in insertion order.
    /// # Examples
    /// See [`AttachmentState`] for text attachment and consumption.
    pub fn take_text(&mut self) -> Vec<PendingTextPaste> {
        std::mem::take(&mut self.text)
    }

    /// Clear image and text buffers after a successful steering submission.
    /// # Examples
    /// ```
    /// let mut state = codetether_paste_buffer::AttachmentState::default();
    /// state.attach_text("hello".into());
    /// state.clear();
    /// assert!(state.text.is_empty());
    /// assert!(state.images.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.images.clear();
        self.text.clear();
    }
}
