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
    let head = &payload[..payload.floor_char_boundary(8)];
    let tail_start = payload.len().saturating_sub(4);
    let tail = &payload[tail_start..];
    format!("bedrock-api-key-{head}…{tail}")
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
