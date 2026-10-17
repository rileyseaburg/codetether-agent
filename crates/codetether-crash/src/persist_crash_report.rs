//! Crash reporting persist crash report.
use super::*;
pub(super) fn persist_crash_report(
    settings: &CrashReporterSettings,
    panic_info: &panic::PanicHookInfo<'_>,
) -> Result<PathBuf> {
    std::fs::create_dir_all(&settings.report_dir)
        .with_context(|| format!("create report dir {}", settings.report_dir.display()))?;

    let report = CrashReport::from_panic_info(settings, panic_info);
    let file_name = format!(
        "{}-{}.json",
        report.occurred_at.format("%Y%m%dT%H%M%S%.3fZ"),
        report.report_id
    );
    let path = settings.report_dir.join(file_name);
    let payload = serde_json::to_string_pretty(&report)?;
    std::fs::write(&path, payload).with_context(|| format!("write report {}", path.display()))?;

    prune_old_reports(&settings.report_dir, MAX_PENDING_REPORTS)?;
    Ok(path)
}
