//! Map native persistence results into stable import outcomes.
use crate::PersistOutcome;
use anyhow::Result;

/// Persist a parsed session under its native JSON filename.
/// # Arguments
/// * `session` — Parsed native session to store.
/// * `data_dir` — Native session storage directory.
/// # Returns
/// Whether storage saved a newer import or retained an existing session.
/// # Errors
/// Propagates native store failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_codex_import::Host>(session: H::Session) -> anyhow::Result<()> {
/// let _outcome = codetether_codex_import::persist_imported_session::<H>(session, std::path::Path::new("sessions")).await?;
/// # Ok(()) }
/// ```
pub async fn persist_imported_session<H: crate::Host>(
    session: H::Session,
    data_dir: &std::path::Path,
) -> Result<PersistOutcome> {
    let path = data_dir.join(format!("{}.json", H::session_id(&session)));
    if !H::persist(session, &path).await? {
        return Ok(PersistOutcome::Unchanged);
    }
    Ok(PersistOutcome::Saved)
}
