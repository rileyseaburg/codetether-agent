/// Compare two byte slices in constant time.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::server::auth::constant_time_eq;
///
/// assert!(constant_time_eq(b"hello", b"hello"));
/// assert!(!constant_time_eq(b"hello", b"world"));
/// ```
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (left, right) in a.iter().zip(b.iter()) {
        diff |= left ^ right;
    }
    diff == 0
}
