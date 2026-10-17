//! Crash reporting prune old reports.
use super::*;
pub(super) fn prune_old_reports(report_dir: &Path, max_reports: usize) -> Result<()> {
    let mut reports = pending_report_paths(report_dir)?;
    if reports.len() <= max_reports {
        return Ok(());
    }

    reports.sort_by_key(|path| {
        std::fs::metadata(path)
            .ok()
            .and_then(|meta| meta.modified().ok())
    });

    let remove_count = reports.len().saturating_sub(max_reports);
    for path in reports.into_iter().take(remove_count) {
        if let Err(err) = std::fs::remove_file(&path) {
            tracing::warn!(path = %path.display(), error = %err, "Failed pruning old crash report");
        }
    }

    Ok(())
}
