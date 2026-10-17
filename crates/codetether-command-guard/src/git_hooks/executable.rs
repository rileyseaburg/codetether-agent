//! executable in shell hook-bypass detection.
use super::*;

pub(super) fn executable(word: &str) -> &str {
    word.rsplit('/').next().unwrap_or("")
}
