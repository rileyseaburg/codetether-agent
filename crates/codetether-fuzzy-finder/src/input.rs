//! Query editing and terminal key transitions for the finder.
//! Handle a key on the active state, then dispatch its FinderAction in the host.
use crate::{FinderAction, FuzzyFindState};
use crossterm::event::{KeyCode, KeyEvent};
impl FuzzyFindState {
    /// Edit the query or request a host action.
    ///
    /// # Arguments
    /// * `key` — Terminal key event.
    ///
    /// # Returns
    /// Cancel or Confirm for host actions, otherwise Continue.
    ///
    /// # Examples
    /// ```
    /// use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    /// use codetether_fuzzy_finder::{FinderAction, FuzzyFindState};
    /// let mut state = FuzzyFindState::default();
    /// assert_eq!(state.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE)), FinderAction::Cancel);
    /// ```
    pub fn handle_key(&mut self, key: KeyEvent) -> FinderAction {
        match key.code {
            KeyCode::Esc => return FinderAction::Cancel,
            KeyCode::Enter => return FinderAction::Confirm,
            KeyCode::Up => self.move_selection(-1),
            KeyCode::Down => self.move_selection(1),
            KeyCode::Backspace => {
                self.query.pop();
                self.rerank();
            }
            KeyCode::Char(c) => {
                self.query.push(c);
                self.rerank();
            }
            _ => {}
        }
        FinderAction::Continue
    }
}
