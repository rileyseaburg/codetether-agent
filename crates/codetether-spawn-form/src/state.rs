//! Editable spawn form values.
//! Form state and terminal presentation require no application runtime.

use crate::SpawnField;

/// Mutable state for the spawn form modal.
///
/// Tracks the agent name and instructions. Tab cycles between both fields.
///
/// # Examples
///
/// ```
/// use codetether_spawn_form::SpawnFormState;
///
/// let mut form = SpawnFormState::default();
/// assert_eq!(form.active, SpawnFormState::default().active);
/// form.next_field();
/// ```
#[derive(Debug, Clone, Default)]
pub struct SpawnFormState {
    /// Current text in the Name field.
    pub name: String,
    /// Current text in the Instructions field.
    pub instructions: String,
    /// Which field has focus.
    pub active: SpawnField,
}
