//! Descendant collection for cascading reaps.

use std::collections::HashMap;

use crate::PersonaRuntimeState;

/// Collect `persona_id` plus, when `cascade`, all transitive descendants.
///
/// The result is breadth-first and deduplicated, so a malformed parent cycle
/// cannot loop forever.
///
/// # Arguments
/// * `personas` - Persona records indexed by ID.
/// * `persona_id` - Persona to credit or collect.
/// * `cascade` - Whether to include transitive descendants.
///
/// # Returns
/// The result of the policy described above.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::persona::reap_targets::collect_reap_targets;
/// let personas = std::collections::HashMap::new();
/// let result = collect_reap_targets(&personas, "p1", true);
/// assert_eq!(result, ["p1"]);
/// ```
pub fn collect_reap_targets(
    personas: &HashMap<String, PersonaRuntimeState>,
    persona_id: &str,
    cascade: bool,
) -> Vec<String> {
    let mut targets = vec![persona_id.to_string()];
    if !cascade {
        return targets;
    }
    let mut idx = 0usize;
    while idx < targets.len() {
        let current = targets[idx].clone();
        let children: Vec<String> = personas
            .values()
            .filter(|p| p.identity.parent_id.as_deref() == Some(current.as_str()))
            .map(|p| p.identity.id.clone())
            .collect();
        for child in children {
            if !targets.contains(&child) {
                targets.push(child);
            }
        }
        idx += 1;
    }
    targets
}
