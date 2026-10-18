//! clean in shell hook-bypass detection.

pub(super) fn clean(word: &str) -> &str {
    word.trim_matches(['\'', '"', '(', ')'])
}
