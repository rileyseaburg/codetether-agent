//! clean in shell hook-bypass detection.
use super::*;

pub(super) fn clean(word: &str) -> &str {
    word.trim_matches(['\'', '"', '(', ')'])
}
