//! Cancellation cleanup for direct swarm observers.

use super::observer::Observer;
use crate::tui::swarm_view::SwarmEvent;

impl Drop for Observer {
    fn drop(&mut self) {
        if !self.terminal {
            let _ = self.events.try_send(SwarmEvent::Error(
                "Direct swarm cancelled before completion".into(),
            ));
        }
    }
}
