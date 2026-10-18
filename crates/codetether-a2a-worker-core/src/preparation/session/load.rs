use super::Loader;
use anyhow::{Context, Result};
/// Load a resumed session or fall back to context resolution when allowed.
/// # Arguments
/// `resume_session_id` selects resume; `context_id` selects fallback;
/// `require_resume` forbids fallback after a resume failure.
/// # Returns
/// The native session selected by the existing loading policy.
/// # Errors
/// Propagates required-resume failures with verification context, or fallback errors.
/// # Examples
/// ```no_run
/// # async fn load<L: codetether_a2a_worker_core::preparation::session::Loader>() -> anyhow::Result<()> {
/// let _session = codetether_a2a_worker_core::preparation::session::load_ids::<L>(None, None, false).await?;
/// # Ok(()) }
/// ```

pub async fn load_ids<L: Loader>(
    resume_session_id: Option<&str>,
    context_id: Option<&str>,
    require_resume: bool,
) -> Result<L::Session> {
    if let Some(session_id) = resume_session_id {
        match L::resume(session_id).await {
            Ok(session) => return Ok(session),
            Err(error) if require_resume => {
                return Err(error).context("Verified author session is unavailable");
            }
            Err(_) => {}
        }
    }
    L::resolve(context_id).await
}
