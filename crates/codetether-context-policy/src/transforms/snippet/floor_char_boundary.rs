//! floor char boundary.
use super::*;

/// Round `idx` down to the nearest UTF-8 character boundary. Returns 0
/// if no valid boundary exists below `idx`.
pub(super) fn floor_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}
