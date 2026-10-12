//! State container for the interactive spawn form.

/// Which form field currently has input focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpawnField {
    /// Agent name (required).
    #[default]
    Name,
    /// Free-text system instructions.
    Instructions,
}

/// Mutable state for the spawn form modal.
///
/// Tracks the agent name and instructions. Tab cycles between both fields.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::spawn_form::SpawnFormState;
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

impl SpawnFormState {
    /// Advance focus to the next field (wraps around).
    pub fn next_field(&mut self) {
        self.active = match self.active {
            SpawnField::Name => SpawnField::Instructions,
            SpawnField::Instructions => SpawnField::Name,
        };
    }

    /// Insert a character into the active field.
    pub fn insert_char(&mut self, c: char) {
        self.active_mut().push(c);
    }

    /// Delete the last character from the active field.
    pub fn backspace(&mut self) {
        self.active_mut().pop();
    }

    /// Borrow the text of the currently active field.
    pub fn active_str(&self) -> &str {
        match self.active {
            SpawnField::Name => &self.name,
            SpawnField::Instructions => &self.instructions,
        }
    }

    fn active_mut(&mut self) -> &mut String {
        match self.active {
            SpawnField::Name => &mut self.name,
            SpawnField::Instructions => &mut self.instructions,
        }
    }

    /// Assemble the argument string for `handle_spawn_command`.
    ///
    /// Returns `None` if the name field is empty.
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
