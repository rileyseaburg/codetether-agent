//! Focused field selection and access.
//! Form state and terminal presentation require no application runtime.

use crate::{SpawnField, SpawnFormState};
impl SpawnFormState {
    /// Advance focus to the next field (wraps around).
    ///
    /// # Returns
    /// Changes focus, wrapping after the final field.
    ///
    /// # Examples
    /// ```
    /// let mut form = codetether_spawn_form::SpawnFormState::default();
    /// form.next_field();
    /// assert_eq!(form.active, codetether_spawn_form::SpawnField::Instructions);
    /// ```
    pub fn next_field(&mut self) {
        self.active = match self.active {
            SpawnField::Name => SpawnField::Instructions,
            SpawnField::Instructions => SpawnField::Name,
        };
    }

    /// Borrow the text of the currently active field.
    ///
    /// # Returns
    /// The borrowed text in the focused field.
    ///
    /// # Examples
    /// ```
    /// let mut form = codetether_spawn_form::SpawnFormState::default();
    /// assert_eq!(form.active_str(), "");
    /// ```
    pub fn active_str(&self) -> &str {
        match self.active {
            SpawnField::Name => &self.name,
            SpawnField::Instructions => &self.instructions,
        }
    }

    pub(crate) fn active_mut(&mut self) -> &mut String {
        match self.active {
            SpawnField::Name => &mut self.name,
            SpawnField::Instructions => &mut self.instructions,
        }
    }
}
