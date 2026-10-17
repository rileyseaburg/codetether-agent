//! Shared auto-chat request policy.
use super::*;

/// Append missing capabilities required by relay participants.
///
/// # Arguments
/// * `capabilities` - Mutable advertised capability list.
///
/// # Returns
/// Nothing; existing entries and their order are retained.
///
/// # Examples
/// ```
/// use codetether_autochat_core::ensure_required_relay_capabilities;
/// let mut capabilities = Vec::new();
/// ensure_required_relay_capabilities(&mut capabilities);
/// assert!(capabilities.iter().any(|value| value == "relay"));
/// ```
pub fn ensure_required_relay_capabilities(capabilities: &mut Vec<String>) {
    for required in REQUIRED_RELAY_CAPABILITIES {
        if !capabilities.iter().any(|cap| cap == required) {
            capabilities.push(required.to_string());
        }
    }
}
