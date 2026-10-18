//! Native model-picker view and session preference bindings.
use crate::ModelPickerState;
/// Interactive shell operations surrounding the shared picker state.
/// # Implementors
/// Native clients provide their picker state and existing chat-return behavior.
pub trait View {
    /// Borrow the model-picker state.
    fn picker(&mut self) -> &mut ModelPickerState;
    /// Select the model-picker view.
    fn show_picker(&mut self);
    /// Replace the status line.
    fn set_status(&mut self, status: String);
    /// Restore the chat view using the client's existing behavior.
    fn return_to_chat(&mut self);
}
/// Session model preference used when confirming a picker selection.
/// # Implementors
/// Native sessions set their existing model metadata field.
pub trait Session {
    /// Set the selected provider-qualified model reference.
    fn set_model(&mut self, model: String);
}
