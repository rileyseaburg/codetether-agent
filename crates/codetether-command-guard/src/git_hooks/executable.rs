//! executable in shell hook-bypass detection.

pub(super) fn executable(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or("")
}
