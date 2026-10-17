//! Execute the conflict resolver with the original tool filter and limits.
use super::*;
impl RalphLoop {
    pub(super) async fn execute_conflict_resolution(
        provider: &Arc<dyn Provider>,
        model: &str,
        working_dir: &Path,
        story_id: &str,
        prompt: &str,
        bus: Option<Arc<AgentBus>>,
    ) -> anyhow::Result<String> {
        // Build system prompt
        let system_prompt = crate::agent::builtin::build_system_prompt(working_dir);

        // Create tool registry
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(provider), model.to_string());

        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(
            "Conflict resolver starting with {} tools",
            tool_definitions.len()
        );

        // Run the resolver with smaller limits (conflicts should be quick to resolve)
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
            Arc::clone(provider),
            model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry,
            15,  // max 15 steps for conflict resolution
            120, // 2 min per-step timeout (resets on progress)
            None,
            String::new(),
            bus.clone(),
            Some(working_dir.to_path_buf()),
        )
        .await?;

        info!(
            story_id = %story_id,
            steps = steps,
            tool_calls = tool_calls,
            "Conflict resolver completed"
        );

        Ok(output)
    }
}
