//! Create the run that will receive the CLI approval decision.
use codetether_okr::OkrRun;
use uuid::Uuid;
/// Create and submit a fresh execution run for CLI confirmation.
/// # Arguments
/// * `okr_id` - Objective identity to associate with the run.
/// # Returns
/// A timestamp-named run after the existing submission attempt.
/// # Examples
/// ```
/// # fn main() {
/// let run = codetether_relay_okr::cli_approval::new_run(uuid::Uuid::nil());
/// assert_eq!(run.okr_id, uuid::Uuid::nil());
/// # }
/// ```
pub fn new_run(okr_id: Uuid) -> OkrRun {
    let mut run = OkrRun::new(
        okr_id,
        format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
    );
    let _ = run.submit_for_approval();

    run
}
