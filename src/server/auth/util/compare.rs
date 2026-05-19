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
    let len_diff = (a.len() ^ b.len()) as u8;
    let mut diff = len_diff;
    for i in 0..a.len().max(b.len()) {
        let left = a.get(i).copied().unwrap_or(0);
        let right = b.get(i).copied().unwrap_or(0);
        diff |= left ^ right;
    }
    diff == 0
}
