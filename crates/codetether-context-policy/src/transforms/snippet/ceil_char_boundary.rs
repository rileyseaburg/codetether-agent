//! ceil char boundary.
use super::*;

/// Round `idx` up to the nearest UTF-8 character boundary.
pub(super) fn ceil_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx < s.len() && !s.is_char_boundary(idx) {
        idx += 1;
    }
    idx
}
