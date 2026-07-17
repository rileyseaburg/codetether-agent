//! Constant-time validation for configured and discovered peer tokens.

pub(super) fn accepted(provided: &str, configured: &str) -> bool {
    let configured_match = constant_time_eq(provided.as_bytes(), configured.as_bytes());
    let discovered_match = constant_time_eq(
        provided.as_bytes(),
        crate::a2a::collaboration_token::local().as_bytes(),
    );
    configured_match | discovered_match
}

/// Compares token bytes without leaking a matching prefix or length.
///
/// # Arguments
///
/// * `left` — First token byte sequence.
/// * `right` — Second token byte sequence.
///
/// # Returns
///
/// `true` only when both byte sequences are identical.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::a2a::server_auth::constant_time_eq;
///
/// assert!(constant_time_eq(b"token", b"token"));
/// assert!(!constant_time_eq(b"token", b"other"));
/// ```
pub fn constant_time_eq(left: &[u8], right: &[u8]) -> bool {
    let mut difference = (left.len() ^ right.len()) as u8;
    for index in 0..left.len().max(right.len()) {
        difference |=
            left.get(index).copied().unwrap_or(0) ^ right.get(index).copied().unwrap_or(0);
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    #[test]
    fn accepts_configured_or_discovered_capability() {
        assert!(super::accepted("configured", "configured"));
        assert!(super::accepted(
            crate::a2a::collaboration_token::local(),
            "configured"
        ));
        assert!(!super::accepted("intruder", "configured"));
    }
}
