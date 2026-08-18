//! Descendant collection for cascading reaps.

use std::collections::HashMap;

use super::PersonaRuntimeState;

/// Collect `persona_id` plus, when `cascade`, all transitive descendants.
///
/// The result is breadth-first and deduplicated, so a malformed parent cycle
/// cannot loop forever.
pub(super) fn collect_reap_targets(
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
