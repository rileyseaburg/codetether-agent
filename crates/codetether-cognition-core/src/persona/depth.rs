//! Spawn-depth limit enforcement.

use anyhow::{Result, anyhow};

use crate::PersonaPolicy;

/// Enforce the effective spawn-depth limit for a new persona.
///
/// The inherited parent policy wins over the child's own policy, so a child
/// cannot widen its own depth allowance.
///
/// # Errors
///
/// Returns an error when `depth` exceeds the effective limit.
///
/// # Arguments
/// * `depth` - Requested lineage depth.
/// * `inherited` - Optional inherited parent policy.
/// * `policy` - New persona policy used if there is no inherited policy.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::persona::depth::check_depth;
/// let policy = PersonaPolicy::default();
/// let result = check_depth(0, None, &policy);
/// assert!(result.is_ok());
/// ```
pub fn check_depth(
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
