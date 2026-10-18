//! Mask a bearer token for safe display (default output path).

/// Return a masked representation showing only the prefix and suffix.
///
/// `bedrock-api-key-` prefix is preserved so the user can identify the token
/// type; only the first 8 and last 4 chars of the payload are shown.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::auth_bedrock::mask::mask_token;
/// let m = mask_token("bedrock-api-key-abcdefghijklmnop");
/// assert!(m.starts_with("bedrock-api-key-"));
/// assert!(m.contains("…"));
/// ```
pub fn mask_token(token: &str) -> String {
    let prefix_len = "bedrock-api-key-".len();
    if token.len() <= prefix_len + 12 {
        return format!("{token}…");
    }
    let payload = &token[prefix_len..];
    let head_end = floor_char_boundary(payload, 8);
    let head = &payload[..head_end];
    let tail_start = floor_char_boundary(payload, payload.len().saturating_sub(4));
    let tail = &payload[tail_start..];
    format!("bedrock-api-key-{head}…{tail}")
}

fn floor_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masks_long_token() {
        let m = mask_token("bedrock-api-key-abcdefghijklmnopqrstuv");
        assert_eq!(m, "bedrock-api-key-abcdefgh…stuv");
    }

    #[test]
    fn short_token_ellipsis_only() {
        let m = mask_token("bedrock-api-key-short");
        assert!(m.ends_with('…'));
    }
}
