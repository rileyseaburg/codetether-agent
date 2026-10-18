//! Command arguments assembled from form values.
//! Form state and terminal presentation require no application runtime.

use crate::SpawnFormState;
impl SpawnFormState {
    /// Assemble the argument string for `handle_spawn_command`.
    ///
    /// Returns `None` if the name field is empty.
    ///
    /// # Returns
    /// Arguments containing the trimmed name and instructions, or `None` for an empty name.
    ///
    /// # Examples
    /// ```
    /// let mut form = codetether_spawn_form::SpawnFormState::default();
    /// assert_eq!(form.to_args(), None);
    /// ```
    pub fn to_args(&self) -> Option<String> {
        let name = self.name.trim();
        if name.is_empty() {
            return None;
        }
        let mut parts = vec![name.to_string()];
        if !self.instructions.trim().is_empty() {
            parts.push(self.instructions.trim().to_string());
        }
        Some(parts.join(" "))
    }
}
