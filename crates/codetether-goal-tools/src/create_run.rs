//! Validation and persistence for `create_goal`.

use super::{Host, context, create::Args, response};
use anyhow::Result;
use chrono::Utc;
use codetether_goal_core::{GoalRuntimeUpdate, GoalSourceKind, GoalStatus, TaskEvent};
use codetether_tool_core::ToolResult;

/// Create a validated, uncapped goal in the invoking session.
/// # Arguments
/// * `args` - Objective and trusted session context.
/// # Returns
/// Current goal JSON or a validation/unfinished-goal refusal.
/// # Errors
/// Returns missing-session or journal/runtime errors.
/// # Examples
/// ```no_run
/// async fn create<H: codetether_goal_tools::Host>() -> anyhow::Result<()> {
///     let _result = codetether_goal_tools::create::<H>(codetether_goal_tools::CreateArgs { objective: "Ship export".into(), session_id: Some("session".into()) }).await?;
///     Ok(())
/// }
/// ```
pub async fn run<H: Host>(mut args: Args) -> Result<ToolResult> {
    if let Some(error) = super::create_validate::apply(&mut args) {
        return Ok(error);
    }
    let session_id = context::session_id(args.session_id)?;
    let (log, state) = H::current(&session_id).await?;
    if state.goal.is_some_and(|goal| !goal.status.is_terminal()) {
        return Ok(ToolResult::error(
            "cannot create a new goal because this session has an unfinished goal; complete the existing goal first",
        ));
    }
    let now = Utc::now();
    let goal_id = uuid::Uuid::new_v4().to_string();
    log.append(&TaskEvent::GoalSet {
        at: now,
        goal_id: goal_id.clone(),
        objective: args.objective,
        success_criteria: Vec::new(),
        forbidden: Vec::new(),
        source_session_id: session_id.clone(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    })
    .await?;
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: now,
        goal_id,
        objective: None,
        status: Some(GoalStatus::Active),
        token_budget: None,
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await?;
    let (_, state) = H::current(&session_id).await?;
    Ok(response::result(&state))
}
