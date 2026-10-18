//! Terminal key editing and host action selection.
//! Form state and terminal presentation require no application runtime.

use crate::{FormAction, SpawnFormState};
use crossterm::event::{KeyCode, KeyEvent};
impl SpawnFormState {
    /// Handle a key, returning any action that needs the application runtime.
    ///
    /// # Arguments
    /// * `key` — Terminal key event.
    ///
    /// # Returns
    /// A submit/cancel request or Continue after editing or ignoring the key.
    ///
    /// # Examples
    /// ```
    /// use codetether_spawn_form::{FormAction, SpawnFormState};
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// let mut form = SpawnFormState::default();
    /// assert_eq!(form.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)), FormAction::Cancel);
    /// ```
    pub fn handle_key(&mut self, key: KeyEvent) -> FormAction {
        match key.code {
            KeyCode::Esc => FormAction::Cancel,
            KeyCode::Tab | KeyCode::BackTab => {
                self.next_field();
                FormAction::Continue
            }
            KeyCode::Enter => FormAction::Submit,
            KeyCode::Backspace => {
                self.backspace();
                FormAction::Continue
            }
            KeyCode::Char(c) => {
                self.insert_char(c);
                FormAction::Continue
            }
            _ => FormAction::Continue,
        }
    }
}
