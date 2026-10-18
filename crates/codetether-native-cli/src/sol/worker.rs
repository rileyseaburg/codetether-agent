//! Session-backed execution by the model selected to write code.

use anyhow::Result;
use std::path::PathBuf;

use super::Session;
use codetether_cli_options::RunArgs;
use codetether_sol_workflow as model;

/// Stateful coding worker shared across implementation and repair turns.
/// # Examples
/// Create workers through the native Sol host, which applies run policy first.
/// ```no_run
/// # async fn example<S: codetether_native_cli::sol::Session, V: codetether_validation::Host + Default + Send>(args: &codetether_cli_options::RunArgs) -> anyhow::Result<()> {
/// use codetether_sol_workflow::Host;
/// let worker = codetether_native_cli::sol::Host::<S, V>::start(args, ".".into()).await?;
/// # Ok(()) }
/// ```
pub struct Worker<S> {
    pub(super) session: S,
    workspace: PathBuf,
    max_steps: Option<usize>,
    resumes: Option<usize>,
}

impl<S: Session> Worker<S> {
    /// Start an isolated coding session for the explicit non-Sol worker model.
    pub(super) async fn start(args: &RunArgs, workspace: PathBuf) -> Result<Self> {
        let config = codetether_run_policy::load(args).await;
        let mut session = S::create().await?;
        session.set_agent_name("sol-worker");
        session.apply_config(&config);
        session.set_model(model::worker_model(args)?);
        codetether_run_policy::apply_session_policy(&mut session, args)?;
        Ok(Self {
            session,
            workspace,
            max_steps: args.max_steps,
            resumes: args.auto_continue_until,
        })
    }

    /// Run a tool-using worker turn and return its final response text.
    pub(super) async fn execute(&mut self, prompt: &str) -> Result<String> {
        let result = codetether_run_resume::execute_prompt_with_resume(
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
