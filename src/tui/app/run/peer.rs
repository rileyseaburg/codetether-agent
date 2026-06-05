use std::sync::Arc;

use crate::a2a::spawn::{A2APeerHandle, SpawnOptions};
use crate::bus::AgentBus;

pub(super) struct PeerEndpoint {
    pub ready: bool,
    _handle: Option<A2APeerHandle>,
}

pub(super) async fn start(options: Option<SpawnOptions>, bus: Arc<AgentBus>) -> PeerEndpoint {
    let Some(options) = options else {
        return PeerEndpoint {
            ready: false,
            _handle: None,
        };
    };
    match crate::a2a::spawn::start_a2a_in_background(options, bus).await {
        Ok(handle) => {
            tracing::info!(
                agent = %handle.agent_name,
                bind_addr = %handle.bind_addr,
                public_url = %handle.public_url,
                "TUI A2A peer endpoint ready"
            );
            PeerEndpoint {
                ready: true,
                _handle: Some(handle),
            }
        }
        Err(err) => {
            tracing::error!(error = %err, "Failed to start TUI A2A peer; continuing without it");
            PeerEndpoint {
                ready: false,
                _handle: None,
            }
        }
    }
}
