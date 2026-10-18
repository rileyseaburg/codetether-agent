//! Fallback objective and measurable relay outcomes.
use crate::truncate_with_ellipsis;
use codetether_okr::{KeyResult, Okr};
use uuid::Uuid;

/// Create fallback measurable objectives for relay execution.
/// # Arguments
/// * `okr_id` - Identity assigned to the objective.
/// * `task` - User task used in the title and description.
/// # Returns
/// An objective with completion, handoff and error key results.
/// # Examples
/// ```
/// let objective = codetether_relay_plan::default_relay_okr_template(uuid::Uuid::nil(), "repair");
/// assert_eq!(objective.key_results.len(), 3);
/// ```
pub fn default_relay_okr_template(okr_id: Uuid, task: &str) -> Okr {
    let mut okr = Okr::new(
        format!("Relay: {}", truncate_with_ellipsis(task, 60)),
        format!("Execute relay task: {}", task),
    );
    okr.id = okr_id;

    // Default key results (fallback only)
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Relay completes all rounds",
        100.0,
        "%",
    ));
    okr.add_key_result(KeyResult::new(
        okr_id,
        "Team produces actionable handoff",
        1.0,
        "count",
    ));
    okr.add_key_result(KeyResult::new(okr_id, "No critical errors", 0.0, "count"));
    okr
}
