//! Launch one targeted repair attempt using the original reduced step limit.
use super::*;
impl RalphLoop {
    pub(super) async fn repair_parallel_story(
        provider: &Arc<dyn Provider>,
        model: &str,
        story: &UserStory,
        story_working_dir: &PathBuf,
        bridge_tx: Option<mpsc::Sender<SwarmEvent>>,
        bus: Option<Arc<AgentBus>>,
        qg_attempt: usize,
        max_quality_retries: usize,
        error_output: &str,
    ) -> anyhow::Result<String> {
        // Feed errors back to agent for self-repair
        warn!(
            story_id = %story.id,
            attempt = qg_attempt + 1,
            max_retries = max_quality_retries,
            "Quality gates failed, spawning repair agent"
        );

        let repair_prompt = codetether_ralph_core::presentation::build_repair_prompt(
            qg_attempt,
            max_quality_retries,
            story,
            error_output,
            story_working_dir,
        );
        // Repair agent gets fewer steps
        let repair_result = Self::call_llm_static(
            provider,
            model,
            &repair_prompt,
            story_working_dir,
            bridge_tx,
            story.id.clone(),
            bus,
            20, // fewer steps for targeted repair
        )
        .await;

        repair_result
    }
}
