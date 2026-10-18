//! Default quantitative relay objective and key results.
use codetether_okr::{KeyResult, Okr};
use uuid::Uuid;

/// Build a default relay objective with three measurable key results.
/// # Arguments
/// * `okr_id` - ID shared by the objective and its key results.
/// * `task` - Original relay task text.
/// # Returns
/// An objective describing completion, handoff and critical-error targets.
/// # Examples
/// ```
/// let okr = codetether_relay_okr::default_relay_okr_template(uuid::Uuid::nil(), "build feature");
/// assert_eq!(okr.key_results.len(), 3);
/// ```
pub fn default_relay_okr_template(okr_id: Uuid, task: &str) -> Okr {
    let mut okr = Okr::new(
        format!("Relay: {}", task.to_string()),
        format!("Execute relay task: {task}"),
    );
    okr.id = okr_id;

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
