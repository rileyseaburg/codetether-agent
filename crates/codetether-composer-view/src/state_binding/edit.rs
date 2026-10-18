//! Application-local edit methods for the shared composer.
/// Internal composer-state method declaration.
#[doc(hidden)]
#[macro_export]
macro_rules! __app_composer_edit {
    ($state:ty) => {
        impl $state {
            pub(super) fn composer(&mut self) -> $crate::interactive::Composer<'_> {
                $crate::interactive::Composer {
                    input: &mut self.input,
                    cursor: &mut self.input_cursor,
                    scroll: &mut self.input_scroll,
                    mode: &mut self.input_mode,
                    completion: &mut self.completion,
                    history: &mut self.history,
                }
            }
            pub fn insert_char(&mut self, c: char) {
                self.composer().insert_char(c);
            }
            pub fn insert_text(&mut self, text: &str) {
                self.composer().insert_text(text);
            }
            pub fn delete_backspace(&mut self) {
                self.composer().delete_backspace();
            }
            pub fn delete_forward(&mut self) {
                self.composer().delete_forward();
            }
            pub fn clear_input(&mut self) {
                self.composer().clear_input();
            }
        }
    };
}
