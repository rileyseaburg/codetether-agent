//! Render protected checkpoint excerpts for the next model context.
/// Render the snapshot's protected constraints with their absolute turn numbers.
/// # Arguments
/// `snapshot` supplies the instance-local checkpoint and its retained excerpts.
/// # Returns
/// One formatted line per retained constraint in checkpoint order.
/// # Panics
/// Panics if the checkpoint mutex is poisoned, matching native session behavior.
/// # Examples
/// ```no_run
/// # fn example(snapshot: &impl codetether_session_store::Snapshot) {
/// let _lines = codetether_session_store::constraints::prefix(snapshot);
/// # }
/// ```
pub fn prefix(snapshot: &impl crate::Snapshot) -> Vec<String> {
    snapshot
        .state()
        .0
        .lock()
        .unwrap()
        .constraints
        .iter()
        .map(|(seq, text)| format!("- turn {seq}: {text}"))
        .collect()
}
