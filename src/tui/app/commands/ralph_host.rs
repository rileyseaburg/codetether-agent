//! Native Ralph loop binding for shared interactive command execution.
use crate::ralph::RalphLoop;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, events::RalphEvent};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;
pub(super) struct AppHost;
impl codetether_ralph_cli::interactive::Host for AppHost {
    type Run = RalphLoop;
    async fn create(
        path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        events: mpsc::Sender<RalphEvent>,
    ) -> anyhow::Result<RalphLoop> {
        RalphLoop::new(path, provider, model, config)
            .await
            .map(|run| run.with_event_tx(events))
    }
    async fn run(run: &mut RalphLoop) -> anyhow::Result<()> {
        run.run().await.map(|_| ())
    }
}
