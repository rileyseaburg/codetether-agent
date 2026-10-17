//! Execute one relay agent and restore its session before propagating errors.
use super::*;
impl RalphLoop {
    pub(super) async fn run_relay_turn(
        registry: &Arc<ProviderRegistry>,
        sessions: &mut HashMap<String, Session>,
        baton: &str,
        to: &str,
        story_id: &str,
        ralph_tx: &Option<mpsc::Sender<RalphEvent>>,
    ) -> anyhow::Result<String> {
        let Some(mut session) = sessions.remove(to) else {
            anyhow::bail!("Relay agent @{to} session unavailable for story {story_id}");
        };

        let (event_tx, mut event_rx) = mpsc::channel::<SessionEvent>(256);
        let registry_clone = registry.clone();
        let baton_clone = baton.to_string();

        let join = tokio::spawn(async move {
            let result = session
                .prompt_with_events(&baton_clone, event_tx, registry_clone)
                .await;
            (session, result)
        });

        Self::forward_relay_events(&join, &mut event_rx, ralph_tx, to, story_id).await;
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
}
