//! Text and cursor reset for the composer.
use crate::InputEdit;
impl InputEdit<'_> {
    /// Empty the text and return the character cursor to zero.
    pub fn clear(&mut self) {
        self.text.clear();
        *self.cursor = 0;
    }
}
