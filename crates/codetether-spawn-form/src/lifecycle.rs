//! Form opening and cancellation state updates.
use crate::SpawnFormState;
/// Reset and open the form with its keyboard help text.
/// # Arguments
/// Receives the optional form slot and user-visible status string.
/// # Returns
/// Writes a fresh form state and the existing help status.
/// # Examples
/// ```
/// let (mut form, mut status) = (None, String::new());
/// codetether_spawn_form::lifecycle::open(&mut form, &mut status);
/// assert!(form.is_some());
/// ```
pub fn open(form: &mut Option<SpawnFormState>, status: &mut String) {
    *form = Some(SpawnFormState::default());
    *status = "Spawn agent form — Tab: next field, Enter: submit, Esc: cancel".to_string();
}
/// Close the form and show cancellation status.
/// # Arguments
/// Receives the optional form slot and user-visible status string.
/// # Returns
/// Clears the form and writes the existing cancellation status.
/// # Examples
/// ```
/// let (mut form, mut status) = (Some(Default::default()), String::new());
/// codetether_spawn_form::lifecycle::close(&mut form, &mut status);
/// assert!(form.is_none());
/// ```
pub fn close(form: &mut Option<SpawnFormState>, status: &mut String) {
    *form = None;
    *status = "Spawn form cancelled".to_string();
}
