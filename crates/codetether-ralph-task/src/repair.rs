//! Launch one targeted repair attempt using the original reduced step limit.
use crate::*;
pub(crate) async fn repair_parallel_story<H: Host>(
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
    warn!(target: "codetether_agent::ralph::ralph_loop::parallel_repair",
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
    let repair_result = codetether_ralph_agent::parallel::<H::Agent>(
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
