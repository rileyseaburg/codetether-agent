//! Execute one relay agent and restore its session before propagating errors.
use super::*;
pub(crate) async fn run_relay_turn<H: Host>(
    registry: &Arc<H::Registry>,
    sessions: &mut HashMap<String, H::Session>,
    baton: &str,
    to: &str,
    story_id: &str,
    ralph_tx: &Option<mpsc::Sender<RalphEvent>>,
) -> anyhow::Result<String> {
    let Some(session) = sessions.remove(to) else {
        anyhow::bail!("Relay agent @{to} session unavailable for story {story_id}");
    };

    let (event_tx, mut event_rx) = mpsc::channel::<SessionEvent<H::Session>>(256);
    let registry_clone = registry.clone();
    let baton_clone = baton.to_string();

    let join =
        tokio::spawn(
            async move { H::prompt(session, baton_clone, event_tx, registry_clone).await },
        );

    forward_relay_events(&join, &mut event_rx, ralph_tx, to, story_id).await;
    let (updated_session, result) = join
        .await
        .map_err(|e| anyhow::anyhow!("Relay agent @{to} task error: {e}"))?;

    // Drain remaining events
    while event_rx.try_recv().is_ok() {}

    sessions.insert(to.to_string(), updated_session);

    let output = result
        .map_err(|e| anyhow::anyhow!("Relay agent @{to} failed on story {story_id}: {e}"))?
        .text;

    Ok(output)
}
