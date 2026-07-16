//! Shared event channel consumed by the TUI tick loop.

use crate::tui::swarm_view::{SwarmEvent, SwarmViewState};
use std::sync::Mutex;
use tokio::sync::mpsc;

lazy_static::lazy_static! {
    static ref CHANNEL: (mpsc::Sender<SwarmEvent>, Mutex<mpsc::Receiver<SwarmEvent>>) = {
        let (sender, receiver) = mpsc::channel(512);
        (sender, Mutex::new(receiver))
    };
}

pub(super) fn sender() -> mpsc::Sender<SwarmEvent> {
    CHANNEL.0.clone()
}

pub(super) fn drain(view: &mut SwarmViewState) -> bool {
    let Ok(mut receiver) = CHANNEL.1.lock() else {
        return false;
    };
    let mut changed = false;
    while let Ok(event) = receiver.try_recv() {
        view.handle_event(event);
        changed = true;
    }
    changed
}
