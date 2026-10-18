//! Normalize model-authored objectives and quantitative key results.
use crate::planned::default_okr_unit;
use crate::{PlannedOkrDraft, default_relay_okr_template};
use codetether_okr::{KeyResult, Okr};
use uuid::Uuid;

/// Normalize a model draft into an objective, falling back when no valid results remain.
/// # Arguments
/// * `okr_id` - ID to assign to the objective and key results.
/// * `task` - Original task used by fallback labels and the default template.
/// * `planned` - Deserialized model draft.
/// # Returns
/// An objective with at most seven considered key results, nonnegative targets
/// and default units for empty unit fields.
/// # Examples
/// ```
/// let draft = serde_json::from_str::<codetether_relay_okr::PlannedOkrDraft>("{}").unwrap();
/// let okr = codetether_relay_okr::okr_from_planned_draft(uuid::Uuid::nil(), "task", draft);
/// assert_eq!(okr.key_results.len(), 3);
/// ```
pub fn okr_from_planned_draft(okr_id: Uuid, task: &str, planned: PlannedOkrDraft) -> Okr {
    let title = if planned.title.trim().is_empty() {
        format!("Relay: {}", task.to_string())
    } else {
        planned.title.trim().to_string()
    };

    let description = if planned.description.trim().is_empty() {
        format!("Execute relay task: {task}")
    } else {
        planned.description.trim().to_string()
    };

    let mut okr = Okr::new(title, description);
    okr.id = okr_id;

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
