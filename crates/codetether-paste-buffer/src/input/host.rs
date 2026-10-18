use crate::PendingTextPaste;
/// Native image and text-composer operations for chat paste ingestion.
/// # Implementors
/// A client owns its attachment queue, input cursor and display state.
pub trait Host {
    /// Consume an image paste or rejection; false means it remains text.
    fn try_attach_image(&mut self, text: &str) -> bool;
    /// Read the current composer input.
    fn input(&self) -> &str;
    /// Select command mode when true, ordinary editing mode otherwise.
    fn set_command_mode(&mut self, command: bool);
    /// Insert text at the current cursor using native editing behavior.
    fn insert_text(&mut self, text: &str);
    /// Set the user-visible paste status.
    fn set_status(&mut self, status: String);
    /// Access the sidecar entries associated with this composer.
    fn pending_pastes(&mut self) -> &mut Vec<PendingTextPaste>;
}
