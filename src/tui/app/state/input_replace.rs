//! Native range-replacement entry point for shared composer transitions.
use std::ops::Range;
impl super::AppState {
    pub fn replace_input_chars(&mut self, range: Range<usize>, replacement: &str) {
        self.composer().replace_input_chars(range, replacement);
    }
}
