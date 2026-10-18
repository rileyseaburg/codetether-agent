//! directory import workflow.
use crate::{CodexImportReport, PersistOutcome, persist_imported_session};
use anyhow::Result;
use codetether_codex_discovery::discover_codex_sessions_for_directory;

/// Import matching workspace archives and count storage outcomes.
/// # Arguments
/// * `dir` — Workspace whose archives should be imported.
/// # Returns
/// Counts of saved and unchanged imports.
/// # Errors
/// Propagates archive discovery, parsing, storage and resume failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_codex_import::Host>() -> anyhow::Result<()> {
/// let _result = codetether_codex_import::import_codex_sessions_for_directory::<H>(std::path::Path::new(".")).await?;
/// # Ok(()) }
/// ```
pub async fn import_codex_sessions_for_directory<H: crate::Host>(
    dir: &std::path::Path,
) -> Result<CodexImportReport> {
    let mut report = CodexImportReport::default();
    for info in discover_codex_sessions_for_directory(dir)? {
        let session = H::parse(&info.path, info.title.as_deref())?;
        match persist_imported_session::<H>(session, &H::sessions_dir()?).await? {
            PersistOutcome::Saved => report.imported += 1,
            PersistOutcome::Unchanged => report.skipped += 1,
        }
    }
    Ok(report)
}
