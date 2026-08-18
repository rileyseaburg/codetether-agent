//! Spawn-depth limit enforcement.

use anyhow::{Result, anyhow};

use super::PersonaPolicy;

/// Enforce the effective spawn-depth limit for a new persona.
///
/// The inherited parent policy wins over the child's own policy, so a child
/// cannot widen its own depth allowance.
///
/// # Errors
///
/// Returns an error when `depth` exceeds the effective limit.
pub(super) fn check_depth(
    depth: u32,
    inherited: Option<&PersonaPolicy>,
    policy: &PersonaPolicy,
) -> Result<()> {
    let limit = inherited
        .map(|p| p.max_spawn_depth)
        .unwrap_or(policy.max_spawn_depth);
    if depth > limit {
        return Err(anyhow!("Spawn depth {depth} exceeds limit {limit}"));
    }
    Ok(())
}
