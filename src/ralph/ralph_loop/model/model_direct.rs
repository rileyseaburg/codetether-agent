//! Execute an instance story while retaining its UI event bridge.
use super::*;
impl RalphLoop {
    /// Call the LLM with a prompt using agentic tool loop
    pub(in crate::ralph::ralph_loop) async fn call_llm(
        &self,
        story_id: &str,
        prompt: &str,
    ) -> anyhow::Result<String> {
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(&self.state.working_dir);

        // Create tool registry with provider for file operations
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(&self.provider), self.model.clone());

        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(target: "codetether_agent::ralph::ralph_loop::model_direct",
            "Ralph agent starting with {} tools in {:?}",
            tool_definitions.len(),
            self.state.working_dir
        );

        // Create event bridge if we have an event channel
        let (bridge_tx, _bridge_handle) = if let Some(ref ralph_tx) = self.event_tx {
            let (tx, handle) = Self::create_swarm_event_bridge(ralph_tx, story_id.to_string());
            (Some(tx), Some(handle))
        } else {
            (None, None)
        };

        // Run the agentic loop with tools
        let (output, steps, tool_calls, exit_reason) = run_agent_loop(
            Arc::clone(&self.provider),
            &self.model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry, // Already an Arc<ToolRegistry>
            30,            // max steps per story (focused implementation)
            180,           // 3 minute timeout per story
            bridge_tx,
            story_id.to_string(),
            self.bus.clone(),
            Some(self.state.working_dir.clone()),
        )
        .await?;

        Self::log_model_exit(story_id, steps, tool_calls, exit_reason);
        Ok(output)
    }
}
