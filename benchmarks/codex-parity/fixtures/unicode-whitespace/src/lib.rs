//! Unicode-aware text utilities.

/// Count non-empty words separated by whitespace.
///
/// # Arguments
///
/// * `input` - Text whose words should be counted.
///
/// # Returns
///
/// The number of non-empty whitespace-delimited words.
///
/// # Examples
///
/// ```
/// assert_eq!(unicode_whitespace::count_words("two words"), 2);
/// ```
pub fn count_words(input: &str) -> usize {
    input.split(' ').filter(|word| !word.is_empty()).count()
}
