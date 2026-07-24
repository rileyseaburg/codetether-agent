//! Session-backed execution by the model selected to write code.

use anyhow::Result;
use std::path::PathBuf;

use super::super::RunArgs;
use super::model;
use crate::session::Session;

/// Stateful coding worker shared across implementation and repair turns.
pub(super) struct Worker {
    pub(super) session: Session,
    workspace: PathBuf,
    max_steps: Option<usize>,
    resumes: Option<usize>,
}

impl Worker {
    /// Start an isolated coding session for the explicit non-Sol worker model.
    pub(super) async fn start(args: &RunArgs, workspace: PathBuf) -> Result<Self> {
        let config = super::super::super::run_config::load(args).await;
        let mut session = Session::new().await?;
        session.set_agent_name("sol-worker");
        session.apply_config(&config, None);
        session.metadata.model = Some(model::worker_model(args)?);
        super::super::super::run_config::apply_session_policy(&mut session, args)?;
        Ok(Self {
            session,
            workspace,
            max_steps: args.max_steps,
            resumes: args.auto_continue_until,
        })
    }

    /// Run a tool-using worker turn and return its final response text.
    pub(super) async fn execute(&mut self, prompt: &str) -> Result<String> {
        let result = super::super::super::run_loop::execute_prompt_with_resume(
            &mut self.session,
            prompt,
            self.max_steps,
            self.resumes,
            &self.workspace,
        )
        .await?;
        Ok(result.text)
    }
}
