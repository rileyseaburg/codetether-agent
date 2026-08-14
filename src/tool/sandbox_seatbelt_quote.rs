//! SBPL string quoting for Seatbelt profile literals.

/// Quote a filesystem path as an SBPL string literal.
///
/// Backslashes and double quotes are escaped so that a path containing them
/// cannot terminate the literal early and inject profile rules.
///
/// # Arguments
///
/// * `value` — Raw path text to embed in a Seatbelt profile.
///
/// # Returns
///
/// The quoted literal, including surrounding double quotes.
pub(super) fn quote(value: &str) -> String {
    let escaped = value.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

#[cfg(test)]
mod tests {
    use super::quote;

    #[test]
    fn quotes_plain_path() {
        assert_eq!(quote("/tmp/work"), "\"/tmp/work\"");
    }

    #[test]
    fn escapes_quote_and_backslash_to_block_rule_injection() {
        assert_eq!(quote("/tmp/a\"b"), "\"/tmp/a\\\"b\"");
        assert_eq!(quote("/tmp/a\\b"), "\"/tmp/a\\\\b\"");
    }
}
