//! Preserve native runtime state and model-edit controller boundaries.
use crate::{Host, VerifierFactory};
use anyhow::Result;
use codetether_goal_core::{GoalEdit, TaskLog, TaskState, control::GoalControlError};
use codetether_native_goals::{control, runtime};
use serde_json::Value;
impl<F> codetether_goal_tools::Host for Host<F> {
    async fn source(
        session: &str,
        objective: &str,
    ) -> Result<Option<codetether_goal_tools::GoalSource>> {
        Ok(codetether_native_goals::source::latest(session)
            .await?
            .map(|source| codetether_goal_tools::GoalSource {
                turn_id: format!("message:{}", source.index),
                text_hash: source.text_hash,
                verbatim: source.text.trim() == objective.trim(),
            }))
    }
    async fn current(session: &str) -> Result<(TaskLog, TaskState)> {
        let (log, state) = runtime::current(session).await?;
        Ok((TaskLog::at(log.path().to_path_buf()), state))
    }
    async fn update(session: &str, edit: GoalEdit) -> Result<Value, GoalControlError> {
        control::update(session, edit).await
    }
}

impl<F: VerifierFactory> codetether_goal_tools::UpdateHost for Host<F> {
    type Verifier = F;
    fn verifier(args: &codetether_goal_tools::UpdateArgs) -> Self::Verifier {
        F::create(
            args.current_model.clone(),
            args.workspace.clone().map_or_else(
                || std::env::current_dir().unwrap_or_else(|_| ".".into()),
                Into::into,
            ),
        )
    }
}
