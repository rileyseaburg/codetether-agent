//! URL detection in the remote auth output stream.
//!
//! AWS SSO device-code flows print a verification URL (often a
//! `verification_uri_complete` with the user code embedded). We scan each
//! line for the first https URL so it can be opened locally.

/// Extract the first `https://` URL appearing in `line`, if any.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::connect::scan_url;
///
/// let line = "Open https://device.sso.us-east-1.amazonaws.com/?user_code=ABCD to continue";
/// assert_eq!(
///     scan_url(line).as_deref(),
///     Some("https://device.sso.us-east-1.amazonaws.com/?user_code=ABCD")
/// );
/// assert!(scan_url("no url here").is_none());
/// ```
pub fn scan_url(line: &str) -> Option<String> {
    let start = line.find("https://")?;
    let rest = &line[start..];
    let end = rest
        .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ')')
        .unwrap_or(rest.len());
    Some(rest[..end].to_string())
}
