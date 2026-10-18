//! Prepare the relay runtime, execute its rounds and perform normal shutdown.
use super::*;
/// Execute a story using a relay team of agents.
///
/// Creates a team of specialist agents (planner, coder, reviewer, etc.),
/// each with its own Session and full tool access, and runs them in a
/// baton-passing relay loop. Each agent operates autonomously within its
/// turn, using tools to read/write code and run commands.
///
/// # Arguments
/// Receives the shared registry, model, story prompt/workspace, optional events
/// and bus, requested team/round limits and optional delegation calibration.
/// # Returns
/// The final handoff text after normal agent shutdown.
/// # Errors
/// Propagates session creation, task join and prompt failures.
/// # Examples
/// ```rust,no_run
/// # use codetether_ralph_relay::{Host, call_relay};
/// # async fn example<H: Host>(registry: &std::sync::Arc<H::Registry>) -> anyhow::Result<String> {
/// call_relay::<H>(registry, "model", "implement the story", std::path::Path::new("."), None, "story-1".into(), None, 2, 1, None).await
/// # }
/// ```
pub async fn call_relay_static<H: Host>(
    registry: &Arc<H::Registry>,
    model: &str,
    prompt: &str,
    working_dir: &Path,
    ralph_tx: Option<mpsc::Sender<RalphEvent>>,
    story_id: String,
    bus: Option<Arc<AgentBus>>,
    max_agents: usize,
    max_rounds: usize,
    delegation: Option<&DelegationState>,
) -> anyhow::Result<String> {
    let max_agents = max_agents.clamp(2, 8);
    let max_rounds = max_rounds.clamp(1, 5);

    let profiles = codetether_ralph_core::relay_profiles::build_implementation_relay_profiles(
        max_agents,
        working_dir,
    );

    // Create relay runtime (use existing bus or create a temporary one)
    let relay_bus = bus.unwrap_or_else(|| Arc::new(AgentBus::new()));
    let relay = ProtocolRelayRuntime::new(relay_bus.clone());

    let mut team = prepare_relay_team::<H>(
        registry, model, &profiles, &relay_bus, delegation, &relay, &story_id, max_rounds,
    )
    .await?;
    let progress = run_relay_rounds::<H>(RelayRoundContext {
        registry,
        team: &mut team,
        relay: &relay,
        ralph_tx: &ralph_tx,
        story_id: &story_id,
        max_rounds,
        prompt,
    })
    .await?;
    Ok(finish_relay(
        &relay,
        &team.ordered_agents,
        progress,
        &story_id,
    ))
}
