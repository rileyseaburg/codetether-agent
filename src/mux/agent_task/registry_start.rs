//! Atomic admission and startup for one mux-owned agent turn.

use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result};

use super::{entry::AgentTask, registry::AgentTaskRegistry};

impl AgentTaskRegistry {
    pub(in crate::mux) fn start(
        &self,
        task_id: &str,
        prompt: &str,
        session_id: Option<&str>,
        max_steps: usize,
        workspace: &Path,
        mux_name: &str,
    ) -> Result<()> {
        super::validation::request(task_id, prompt, session_id, max_steps)?;
        let mut store = self.store.lock().unwrap();
        store.prepare(task_id)?;
        let mut child = super::spawn::child(prompt, session_id, max_steps, workspace, mux_name)?;
        let pid = child.id().context("mux agent process has no pid")?;
        let stdout = child
            .stdout
            .take()
            .context("mux agent stdout unavailable")?;
        let stderr = child
            .stderr
            .take()
            .context("mux agent stderr unavailable")?;
        let task = Arc::new(AgentTask::new(pid));
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
