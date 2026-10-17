//! prefix for the OpenAI-compatible transport.

pub(super) fn safe_char_prefix(input: &str, max_chars: usize) -> String {
    input.chars().take(max_chars).collect()
}
