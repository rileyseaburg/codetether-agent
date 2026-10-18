//! Application-local cursor methods for the shared composer.
/// Internal composer-state method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_composer_cursor {
    ($state:ty) => {
        impl $state {
            pub(crate) fn input_char_count(&self) -> usize {
                $crate::cursor_index::input_char_count(&self.input)
            }

            pub(crate) fn char_to_byte_index(&self, char_index: usize) -> usize {
                $crate::cursor_index::char_to_byte_index(&self.input, char_index)
            }

            pub fn clamp_input_cursor(&mut self) {
                self.composer().clamp_input_cursor();
            }

            #[allow(dead_code)]
            pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
                self.composer().ensure_input_cursor_visible(visible_width);
            }

            pub fn move_cursor_left(&mut self) {
                self.composer().move_cursor_left();
            }

            pub fn move_cursor_right(&mut self) {
                self.composer().move_cursor_right();
            }

            pub fn move_cursor_word_left(&mut self) {
                self.composer().move_cursor_word_left();
            }

            pub fn move_cursor_word_right(&mut self) {
                self.composer().move_cursor_word_right();
            }

            pub fn move_cursor_home(&mut self) {
                self.composer().move_cursor_home();
            }

            pub fn move_cursor_end(&mut self) {
                self.composer().move_cursor_end();
            }
        }
    };
}
