//! Default seed persona request for an empty cognition runtime.
use crate::CreatePersonaRequest;

/// The root persona created when cognition starts with no personas.
///
/// # Arguments
/// None.
///
/// # Returns
/// The default root persona request.
///
/// # Examples
/// ```
/// use codetether_cognition_core::*;
/// use codetether_cognition_core::persona::seed::default_seed_persona;
/// let request = default_seed_persona();
/// assert_eq!(request.name, "root-thinker");
/// ```
pub fn default_seed_persona() -> CreatePersonaRequest {
    CreatePersonaRequest {
        persona_id: Some("root-thinker".to_string()),
        name: "root-thinker".to_string(),
        role: "orchestrator".to_string(),
        charter: "Continuously observe, reflect, test hypotheses, and compress useful insights."
            .to_string(),
        swarm_id: Some("swarm-core".to_string()),
        parent_id: None,
        policy: None,
        tags: vec!["orchestration".to_string()],
    }
}
