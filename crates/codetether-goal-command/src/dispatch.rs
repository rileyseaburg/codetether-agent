//! Goal subcommand parsing and dispatch.
use crate::{Host, auto, budget, show, status, write};
use anyhow::Result;
use codetether_goal_core::GoalEditAction;
/// Execute an explicit human goal command.
/// # Arguments
/// * `session_id` - Session whose goal is being inspected or changed.
/// * `raw` - Text following the goal command, including its subcommand.
/// # Returns
/// User-facing completion or status text.
/// # Errors
/// Returns command-usage, storage, validation or revision-conflict errors.
/// # Examples
/// ```no_run
/// async fn show<H: codetether_goal_command::Host>() -> anyhow::Result<String> {
///     codetether_goal_command::run::<H>("session-id", "show").await
/// }
/// ```
pub async fn run<H: Host>(session_id: &str, raw: &str) -> Result<String> {
    let raw = raw.trim();
    let (verb, tail) = raw
        .split_once(char::is_whitespace)
        .map_or((raw, ""), |(verb, tail)| (verb, tail.trim()));
    match verb {
        "" | "show" | "status" => show::run::<H>(session_id).await,
        "auto" => auto::run::<H>(tail),
        "set" => write::set::<H>(session_id, tail).await,
        "edit" => write::edit::<H>(session_id, tail).await,
        "override" => {
            write::change::<H>(
                session_id,
                GoalEditAction::Override,
                (!tail.is_empty()).then(|| tail.to_string()),
                None,
            )
            .await
        }
        "budget" => budget::run::<H>(session_id, tail).await,
        "reaffirm" => write::reaffirm::<H>(session_id, tail).await,
        "clear" => write::clear::<H>(session_id, tail).await,
        "done" => status::set::<H>(session_id, "complete").await,
        "pause" => status::set::<H>(session_id, "paused").await,
        "resume" => status::set::<H>(session_id, "active").await,
        "execute" => write::change::<H>(session_id, GoalEditAction::Execute, None, None).await,
        other => Err(anyhow::anyhow!(
            "unknown /goal subcommand `{other}`; use set, auto, edit, override, budget, pause, resume, done, clear, or show (/tasks lists work items)"
        )),
    }
}
