//! Bind parallel stories to shared Ralph agent execution.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn call_llm_static(
        provider: &Arc<dyn Provider>,
        model: &str,
        prompt: &str,
        working_dir: &PathBuf,
        event_tx: Option<mpsc::Sender<SwarmEvent>>,
        story_id: String,
        bus: Option<Arc<AgentBus>>,
        max_steps_per_story: usize,
    ) -> anyhow::Result<String> {
        codetether_ralph_agent::parallel::<super::host::Native>(
            provider,
            model,
            prompt,
            working_dir,
            event_tx,
            story_id,
            bus,
            max_steps_per_story,
        )
        .await
    }
}
