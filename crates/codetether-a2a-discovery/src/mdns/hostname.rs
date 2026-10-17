//! DNS-label normalization for mDNS service hostnames.

/// Produces a lowercase RFC 1035 label with a non-empty fallback.
///
/// # Arguments
///
/// * `input` — Agent name to normalize for a `.local.` hostname.
///
/// # Returns
///
/// An ASCII label of at most 63 bytes, or `agent` when no valid characters
/// remain.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_discovery::mdns::sanitize_hostname;
///
/// assert_eq!(sanitize_hostname("My Agent_1"), "my-agent-1");
/// ```
pub fn sanitize_hostname(input: &str) -> String {
    let mut label: String = input
        .chars()
        .map(|character| match character {
            value if value.is_ascii_alphanumeric() => value.to_ascii_lowercase(),
            '-' => '-',
            _ => '-',
        })
        .collect();
    label.truncate(label.len().min(63));
    match label.trim_matches('-') {
        "" => "agent".to_string(),
        value => value.to_string(),
    }
}
#[cfg(test)]
#[path = "hostname_tests.rs"]
mod tests;
