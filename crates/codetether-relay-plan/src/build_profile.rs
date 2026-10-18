//! Normalize planned specialists into runtime relay profiles.
use crate::instructions::relay_instruction_from_plan;
use crate::names::{sanitize_relay_agent_name, slugify_label, unique_relay_agent_name};
use crate::{PlannedRelayProfile, RelayProfile};

/// Normalize a planned specialist, preserving unique runtime names.
/// # Arguments
/// * `profile` - Planner fields before normalization.
/// * `existing` - Names already assigned to the relay.
/// # Returns
/// The resolved specialist, or none if its final name is empty.
/// # Examples
/// ```
/// let planned = serde_json::from_str("{}").unwrap();
/// assert!(codetether_relay_plan::build_runtime_profile_from_plan(planned, &[]).is_some());
/// ```
pub fn build_runtime_profile_from_plan(
    profile: PlannedRelayProfile,
    existing: &[String],
) -> Option<RelayProfile> {
    let specialty = if profile.specialty.trim().is_empty() {
        "generalist".to_string()
    } else {
        profile.specialty.trim().to_string()
    };

    let mission = if profile.mission.trim().is_empty() {
        "Advance the relay with concrete next actions and clear handoffs.".to_string()
    } else {
        profile.mission.trim().to_string()
    };

    let base_name = if profile.name.trim().is_empty() {
        format!("auto-{}", slugify_label(&specialty))
    } else {
        profile.name.trim().to_string()
    };

    let sanitized = sanitize_relay_agent_name(&base_name);
    let name = unique_relay_agent_name(&sanitized, existing);
    if name.trim().is_empty() {
        return None;
    }

    let mut capabilities: Vec<String> = Vec::new();
    let specialty_cap = slugify_label(&specialty);
    if !specialty_cap.is_empty() {
        capabilities.push(specialty_cap);
    }

    for capability in profile.capabilities {
        let normalized = slugify_label(&capability);
        if !normalized.is_empty() && !capabilities.contains(&normalized) {
            capabilities.push(normalized);
        }
    }

    codetether_autochat_core::ensure_required_relay_capabilities(&mut capabilities);

    Some(RelayProfile {
        name: name.clone(),
        instructions: relay_instruction_from_plan(&name, &specialty, &mission),
        capabilities,
    })
}
