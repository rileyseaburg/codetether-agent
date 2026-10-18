//! Complete live events on prompt success or failure.
use crate::Session;
use codetether_cli_options::RunArgs;
use codetether_run_events::LiveEvents;
use codetether_session_events::SessionResult;
use std::path::Path;
pub(super) async fn run<S: Session>(
    session: &mut S,
    args: &RunArgs,
    message: &str,
    workspace_dir: &Path,
    live_events: LiveEvents<S>,
) -> anyhow::Result<SessionResult> {
    Ok(
        match codetether_run_resume::execute_prompt_with_resume_events(
            session,
            message,
            args.max_steps,
            args.auto_continue_until,
            workspace_dir,
            live_events.sender(),
        )
        .await
        {
            Ok(result) => {
                live_events.finish_success(&result.text).await?;
                result
            }
            Err(error) => {
                let message = error.to_string();
                live_events.finish_failure(&message).await?;
                return Err(error);
            }
        },
    )
}
