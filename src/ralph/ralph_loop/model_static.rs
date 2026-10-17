//! Execute a parallel story with its configured step limit.
use super::*;
impl RalphLoop {
    /// Call LLM with agentic tool loop (static version for parallel execution)
    pub(super) async fn call_llm_static(
        provider: &Arc<dyn Provider>,
        model: &str,
        prompt: &str,
        working_dir: &PathBuf,
        event_tx: Option<mpsc::Sender<SwarmEvent>>,
        story_id: String,
        bus: Option<Arc<AgentBus>>,
        max_steps_per_story: usize,
    ) -> anyhow::Result<String> {
        // Build system prompt with AGENTS.md
        let system_prompt = crate::agent::builtin::build_system_prompt(working_dir);

        // Create tool registry with provider for file operations
        let tool_registry =
            ToolRegistry::with_provider_arc(Arc::clone(provider), model.to_string());

        // Filter out 'question' tool - sub-agents must be autonomous, not interactive
        let tool_definitions: Vec<_> = tool_registry
            .definitions()
            .into_iter()
            .filter(|t| t.name != "question")
            .collect();

        info!(
            "Ralph sub-agent starting with {} tools in {:?}",
            tool_definitions.len(),
            working_dir
        );

        // Run the agentic loop with tools
        let max_steps = max_steps_per_story;
        let (output, steps, tool_calls, _exit_reason) = run_agent_loop(
            Arc::clone(provider),
            model,
            &system_prompt,
            prompt,
            tool_definitions,
            tool_registry, // Already an Arc<ToolRegistry>
            max_steps,
            180, // 3 minute timeout per story
            event_tx,
            story_id,
            bus.clone(),
            Some(working_dir.clone()),
        )
        .await?;

        info!(
            "Ralph sub-agent completed: {} steps, {} tool calls",
            steps, tool_calls
        );

        Ok(output)
    }
}
