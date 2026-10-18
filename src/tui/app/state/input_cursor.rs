//! AppState adapters for shared Unicode-aware cursor operations.
impl super::AppState {
    pub(crate) fn input_char_count(&self) -> usize {
        codetether_composer_view::cursor_index::input_char_count(&self.input)
    }

    pub(crate) fn char_to_byte_index(&self, char_index: usize) -> usize {
        codetether_composer_view::cursor_index::char_to_byte_index(&self.input, char_index)
    }

    pub fn clamp_input_cursor(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_move::clamp_input_cursor(
            &self.input,
            self.input_cursor,
        );
    }

    #[allow(dead_code)]
    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        self.input_scroll =
            codetether_composer_view::cursor_visibility::ensure_input_cursor_visible(
                self.input_cursor,
                self.input_scroll,
                visible_width,
            );
    }

    pub fn move_cursor_left(&mut self) {
        self.input_cursor =
            codetether_composer_view::cursor_move::move_cursor_left(self.input_cursor);
    }

    pub fn move_cursor_right(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_move::move_cursor_right(
            &self.input,
            self.input_cursor,
        );
    }

    pub fn move_cursor_word_left(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_word_left::move_cursor_word_left(
            &self.input,
            self.input_cursor,
        );
    }

    pub fn move_cursor_word_right(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_word_right::move_cursor_word_right(
            &self.input,
            self.input_cursor,
        );
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_move::move_cursor_home();
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = codetether_composer_view::cursor_move::move_cursor_end(&self.input);
    }
}
