//! Transform a planned objective into relay execution goals.
use crate::planned_okr::default_okr_unit;
use crate::{PlannedOkrDraft, default_relay_okr_template, truncate_with_ellipsis};
use codetether_okr::{KeyResult, Okr};
use uuid::Uuid;

/// Build a relay objective from planner fields and bounded key results.
/// # Arguments
/// * `okr_id` - Identity assigned to the objective.
/// * `task` - Fallback task description.
/// * `planned` - Candidate objective and key results.
/// # Returns
/// The planned objective or a fallback if no usable key result remains.
/// # Examples
/// ```
/// let draft = serde_json::from_str("{}").unwrap();
/// let objective = codetether_relay_plan::okr_from_planned_draft(uuid::Uuid::nil(), "repair", draft);
/// assert_eq!(objective.key_results.len(), 3);
/// ```
pub fn okr_from_planned_draft(okr_id: Uuid, task: &str, planned: PlannedOkrDraft) -> Okr {
    let title = if planned.title.trim().is_empty() {
        format!("Relay: {}", truncate_with_ellipsis(task, 60))
    } else {
        planned.title.trim().to_string()
    };

    let description = if planned.description.trim().is_empty() {
        format!("Execute relay task: {}", task)
    } else {
        planned.description.trim().to_string()
    };

    let mut okr = Okr::new(title, description);
    okr.id = okr_id;

    // Clamp to a reasonable number of KRs to keep UX readable.
    for kr in planned.key_results.into_iter().take(7) {
        if kr.title.trim().is_empty() {
            continue;
        }
        let unit = if kr.unit.trim().is_empty() {
            default_okr_unit()
        } else {
            kr.unit
        };
        okr.add_key_result(KeyResult::new(
            okr_id,
            kr.title.trim().to_string(),
            kr.target_value.max(0.0),
            unit,
        ));
    }

    if okr.key_results.is_empty() {
        default_relay_okr_template(okr_id, task)
    } else {
        okr
    }
}
