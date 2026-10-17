//! Register the initial in-memory run metadata after persistence.

use codetether_go_monitor::{ACTIVE_GO_RUNS, ActiveGoRun, GoRunPhase};
pub(crate) fn initial(
    okr_id_str: &str,
    task: &str,
    resolved_model: &str,
    prd_filename: &str,
    progress_filename: &str,
) -> String {
    let working_dir = std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| ".".into());

    // Register active run
    let active_run = ActiveGoRun {
        okr_id: okr_id_str.to_owned(),
        task: task.to_owned(),
        model: resolved_model.to_owned(),
        started_at: chrono::Utc::now().to_rfc3339(),
        working_dir: working_dir.clone(),
        prd_filename: prd_filename.to_owned(),
        progress_filename: progress_filename.to_owned(),
        phase: GoRunPhase::Starting,
    };
    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock() {
        runs.insert(okr_id_str.to_owned(), active_run);
    }

    working_dir
}
