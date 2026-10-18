//! Avatar selection for agent names.

use crate::AGENT_AVATARS;

/// Return a deterministic avatar glyph for the given agent name.
///
/// # Arguments
/// * `agent_name` - Name hashed without ASCII case distinctions.
///
/// # Returns
/// A stable glyph borrowed from the avatar palette.
///
/// # Examples
/// ```
/// assert_eq!(codetether_agent_identity::agent_avatar("NAME"), codetether_agent_identity::agent_avatar("name"));
/// ```
pub fn agent_avatar(agent_name: &str) -> &'static str {
    let mut hash: u64 = 2_166_136_261;
    for byte in agent_name.bytes() {
        hash = (hash ^ u64::from(byte.to_ascii_lowercase())).wrapping_mul(16_777_619);
    }
    AGENT_AVATARS[hash as usize % AGENT_AVATARS.len()]
}
