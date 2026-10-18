//! Borrowed text and character-index cursor for one editing operation.
/// Mutates composer text without owning application history or suggestions.
///
/// # Examples
/// ```
/// let mut text = String::new();
/// let mut cursor = 0;
/// codetether_composer_view::InputEdit::new(&mut text, &mut cursor).insert_char('é');
/// assert_eq!((text.as_str(), cursor), ("é", 1));
/// ```
pub struct InputEdit<'a> {
    pub(super) text: &'a mut String,
    pub(super) cursor: &'a mut usize,
}
impl<'a> InputEdit<'a> {
    /// Borrow input text and its cursor, which is measured in Unicode scalars.
    pub fn new(text: &'a mut String, cursor: &'a mut usize) -> Self {
        Self { text, cursor }
    }
}
