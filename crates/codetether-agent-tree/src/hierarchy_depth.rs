//! Parent validation and maximum nesting depth for child creation.
use crate::{HierarchyEntry, MAX_AGENT_DEPTH};
use std::collections::HashMap;

/// Depth a new child of `parent` would occupy (`None` parent → depth 0).
///
/// # Arguments
/// * `agents` — Existing entries keyed by unique display name.
/// * `parent` — Parent name, or None for a child of the main chat.
///
/// # Returns
/// The prospective child depth, or None for an unknown parent.
///
/// # Examples
/// ```
/// # struct Entry;
/// # impl codetether_agent_tree::HierarchyEntry for Entry {
/// #     fn parent_name(&self) -> Option<&str> { None }
/// #     fn depth(&self) -> u8 { 0 }
/// # }
/// let agents = std::collections::HashMap::<String, Entry>::new();
/// assert_eq!(codetether_agent_tree::child_depth(&agents, None), Some(0));
/// ```
pub fn child_depth<E: HierarchyEntry>(
    agents: &HashMap<String, E>,
    parent: Option<&str>,
) -> Option<u8> {
    match parent {
        None => Some(0),
        Some(p) => agents.get(p).map(|a| a.depth() + 1),
    }
}

/// Validate a spawn under `parent`, returning the child depth or an error
/// message (unknown parent or depth cap exceeded).
///
/// # Arguments
/// * `agents` — Existing entries keyed by unique display name.
/// * `parent` — Parent name, or None for a child of the main chat.
///
/// # Returns
/// The prospective depth when the parent exists and the depth cap allows it.
///
/// # Errors
/// Returns a displayable error for an unknown parent or depth above the cap.
///
/// # Examples
/// ```
/// # struct Entry;
/// # impl codetether_agent_tree::HierarchyEntry for Entry {
/// #     fn parent_name(&self) -> Option<&str> { None }
/// #     fn depth(&self) -> u8 { 0 }
/// # }
/// let agents = std::collections::HashMap::<String, Entry>::new();
/// assert_eq!(codetether_agent_tree::validate_spawn(&agents, None), Ok(0));
/// ```
pub fn validate_spawn<E: HierarchyEntry>(
    agents: &HashMap<String, E>,
    parent: Option<&str>,
) -> Result<u8, String> {
    match child_depth(agents, parent) {
        None => Err(format!(
            "Parent agent '{}' not found.",
            parent.unwrap_or("")
        )),
        Some(d) if d > MAX_AGENT_DEPTH => Err(format!(
            "Spawn denied: max agent depth {MAX_AGENT_DEPTH} reached."
        )),
        Some(d) => Ok(d),
    }
}
