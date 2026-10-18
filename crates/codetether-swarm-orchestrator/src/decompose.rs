use super::*;
use codetether_swarm_core::planning::decomposition_prompt;
impl<H: Host> Orchestrator<H> {
    /// Decompose a task and install the generated plan in the coordinator.
    /// # Arguments
    /// `task` is the original task text; `strategy` selects planning behavior.
    /// # Returns
    /// Generated subtasks, or one main task when decomposition is disabled.
    /// # Errors
    /// Returns missing-provider, completion, invalid-plan or repair errors.
    /// # Examples
    /// ```no_run
    /// async fn plan<H: codetether_swarm_orchestrator::Host>(planner: &mut codetether_swarm_orchestrator::Orchestrator<H>) -> anyhow::Result<()> {
    ///     let tasks = planner.decompose("inspect", codetether_swarm_core::DecompositionStrategy::None).await?;
    ///     assert_eq!(tasks.len(), 1);
    ///     Ok(())
    /// }
    /// ```
    pub async fn decompose(
        &mut self,
        task: &str,
        strategy: DecompositionStrategy,
    ) -> Result<Vec<SubTask>> {
        if strategy == DecompositionStrategy::None {
            // Single task, no decomposition
            let subtask = SubTask::new("Main Task", task);
            self.state.insert_task(subtask.clone());
            return Ok(vec![subtask]);
        }

        // Use AI to decompose the task
        let parallel_required =
            self.config.parallel_enabled && strategy != DecompositionStrategy::ByStage;
        let decomposition_prompt = decomposition_prompt::build(
            task,
            strategy,
            self.config.max_subagents,
            parallel_required,
        );

        let provider = H::get(&self.providers, &self.provider)
            .ok_or_else(|| anyhow::anyhow!("Provider {} not found", self.provider))?;

        let temperature = completion::temperature(&self.model);

        let completion = completion::ProviderCompletion {
            provider: provider.as_ref(),
            model: &self.model,
            temperature,
        };
        let tasks = codetether_swarm_core::planning::generate_plan(
            task,
            decomposition_prompt,
            parallel_required,
            &completion,
        )
        .await?;

        self.state.replace_tasks(tasks);

        tracing::info!(
            "Decomposed task into {} subtasks across {} stages",
            self.state.all_subtasks().len(),
            self.max_stage() + 1
        );

        Ok(self.state.all_subtasks().into_iter().cloned().collect())
    }
}
