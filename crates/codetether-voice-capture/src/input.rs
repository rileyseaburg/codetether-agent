//! Frontend operations for consuming completed transcription.
use crate::VoiceSlot;
/// Input state needed when a background transcription completes.
/// # Implementors
/// Frontends provide pending-slot access, text insertion and status updates.
pub trait Input {
    /// Borrow the pending transcription slot if present.
    fn pending(&self) -> Option<&VoiceSlot>;
    /// Clear the frontend's pending transcription handle.
    fn clear_pending(&mut self);
    /// Insert transcribed text using the frontend's normal input path.
    fn insert_text(&mut self, text: &str);
    /// Set the frontend status text.
    fn set_status(&mut self, status: &str);
}
