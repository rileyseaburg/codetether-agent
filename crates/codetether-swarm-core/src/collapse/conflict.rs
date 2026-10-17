use super::*;
pub(super) fn conflict_risk(
    observation: &BranchObservation,
    snapshot: &ObservationSnapshot,
) -> f32 {
    let my_files = snapshot
        .changed_files
        .get(&observation.subtask_id)
        .cloned()
        .unwrap_or_default();

    let overlap_files = snapshot
        .changed_files
        .iter()
        .filter(|(other_id, _)| *other_id != &observation.subtask_id)
        .map(|(_, files)| my_files.intersection(files).count())
        .sum::<usize>();

    let conflict_risk = if my_files.is_empty() {
        0.0
    } else {
        (overlap_files as f32 / my_files.len() as f32).clamp(0.0, 1.0)
    };

    conflict_risk
}
