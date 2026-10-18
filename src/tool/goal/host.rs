//! Preserve native runtime state and model-edit controller boundaries.
use crate::session::tasks::{control, runtime};
use anyhow::Result;
use codetether_goal_core::{GoalEdit, TaskLog, TaskState, control::GoalControlError};
use serde_json::Value;
pub(super) struct Native;
impl codetether_goal_tools::Host for Native {
    async fn current(session: &str) -> Result<(TaskLog, TaskState)> {
        let (log, state) = runtime::current(session).await?;
        Ok((TaskLog::at(log.path().to_path_buf()), state))
    }
    async fn update(session: &str, edit: GoalEdit) -> Result<Value, GoalControlError> {
        control::update(session, edit).await
    }
}

impl codetether_goal_tools::UpdateHost for Native {
    type Verifier = super::verify::LlmVerifier;
    fn verifier(args: &codetether_goal_tools::UpdateArgs) -> Self::Verifier {
        super::verify::LlmVerifier {
            worker_model: args.current_model.clone(),
            workspace: args.workspace.clone().map_or_else(
                || std::env::current_dir().unwrap_or_else(|_| ".".into()),
                Into::into,
            ),
        }
    }
}
