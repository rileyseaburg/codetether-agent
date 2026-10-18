//! Atomic admission and startup for one mux-owned agent turn.

use std::path::Path;
use std::sync::Arc;

use anyhow::Result;

use super::{entry::AgentTask, registry::AgentTaskRegistry};

impl AgentTaskRegistry {
    /// Admit and start one structured agent turn.
    ///
    /// # Arguments
    /// * `task_id` - Unique task identifier, 8–128 ASCII identifier characters.
    /// * `prompt` - Nonempty prompt of at most 64 KiB.
    /// * `session_id` - Optional durable conversation identifier.
    /// * `max_steps` - Turn step budget, from 1 through 250.
    /// * `tool_profile` - Optional supported tool profile (mux_manager).
    /// * `workspace` - Child working directory.
    /// * `mux_name` - Owning mux session and inherited coordination identity.
    /// # Returns
    /// Returns after registering the child and starting output capture.
    /// # Errors
    /// Returns validation, admission, process-spawn or pipe-availability errors.
    /// # Panics
    /// Panics if the store mutex is poisoned or no Tokio runtime is available.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let registry = codetether_mux_agent_task::AgentTaskRegistry::new();
    /// registry.start("task-1234", "Inspect this project", None, 10, None,
    ///     std::path::Path::new("."), "work")?;
    /// # Ok(()) }
    /// ```
    pub fn start(
        &self,
        task_id: &str,
        prompt: &str,
        session_id: Option<&str>,
        max_steps: usize,
        tool_profile: Option<&str>,
        workspace: &Path,
        mux_name: &str,
    ) -> Result<()> {
        super::validation::request(task_id, prompt, session_id, max_steps)?;
        super::validation::tool_profile(tool_profile)?;
        let mut store = self.store.lock().unwrap();
        store.prepare(task_id, mux_name)?;
        let mut child = super::spawn::child(
            prompt,
            session_id,
            max_steps,
            tool_profile,
            workspace,
            mux_name,
        )?;
        let (pid, stdout, stderr) = super::child_io::take(&mut child)?;
        let task = Arc::new(AgentTask::new(pid, mux_name));
        store.insert(task_id.into(), task.clone());
        drop(store);
        let readers = vec![
            super::reader::start(stdout, task.clone()),
            super::reader::start(stderr, task.clone()),
        ];
        super::waiter::start(child, task, readers);
        Ok(())
    }
}
