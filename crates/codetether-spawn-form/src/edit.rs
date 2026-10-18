//! Editing the focused field.
//! Form state and terminal presentation require no application runtime.

use crate::SpawnFormState;
impl SpawnFormState {
    /// Insert a character into the active field.
    ///
    /// # Arguments
    /// * `c` — Character to append.
    ///
    /// # Returns
    /// Appends the character to the focused field.
    ///
    /// # Examples
    /// ```
    /// let mut form = codetether_spawn_form::SpawnFormState::default();
    /// form.insert_char('x');
    /// assert_eq!(form.active_str(), "x");
    /// ```
    pub fn insert_char(&mut self, c: char) {
        self.active_mut().push(c);
    }

    /// Delete the last character from the active field.
    ///
    /// # Returns
    /// Removes the final character, if present.
    ///
    /// # Examples
    /// ```
    /// let mut form = codetether_spawn_form::SpawnFormState::default();
    /// form.backspace();
    /// assert!(form.active_str().is_empty());
    /// ```
    pub fn backspace(&mut self) {
        self.active_mut().pop();
    }
}
