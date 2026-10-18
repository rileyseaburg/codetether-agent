//! Initial session selection for a CLI run.
use crate::SessionSource;
use codetether_cli_options::RunArgs;
use std::path::Path;
/// Select or create a session using the CLI precedence.
/// # Arguments
/// * `args` - Codex import, explicit session and continue-last options.
/// * `workspace_dir` - Scope used for latest-session lookup.
/// # Returns
/// The imported, resumed or newly created session.
/// # Errors
/// Returns import, explicit-resume or creation errors. A failed latest-session
/// lookup falls back to creating a session.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_run_resume::SessionSource>(args: &codetether_cli_options::RunArgs) -> anyhow::Result<()> {
/// let session = codetether_run_resume::select_session::<S>(args, std::path::Path::new(".")).await?;
/// # Ok(()) }
/// ```
pub async fn select_session<S: SessionSource>(
    args: &RunArgs,
    workspace_dir: &Path,
) -> anyhow::Result<S> {
    Ok(if let Some(codex_id) = args.codex_session.clone() {
        tracing::info!("Importing Codex session: {}", codex_id);
        S::import_codex(&codex_id).await?
    } else if let Some(session_id) = args.session.clone() {
        tracing::info!("Continuing session: {}", session_id);
        S::resume(&session_id).await?
    } else if args.continue_session {
        crate::selection_last::resume_or_create::<S>(workspace_dir).await?
    } else {
        let s = S::create().await?;
        tracing::info!("Created new session: {}", s.id());
        s
    })
}
