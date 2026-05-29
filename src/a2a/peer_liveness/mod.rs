//! Liveness tracking for mDNS-discovered A2A peers.
//!
//! mDNS browse events are noisy: a single multi-homed peer announces once
//! per interface, and mdns-sd re-resolves peers periodically. This tracker
//! collapses that noise into three decisions the intake loop needs:
//!
//! * **skip** a duplicate sighting seen within the refresh window,
//! * detect the **first sight** of a peer (for one-shot log + auto-intro),
//! * **expire** peers that have stopped re-announcing so consumers (TUI
//!   registry, UI counts) can age them out instead of showing ghosts.

#[cfg(test)]
mod tests;

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// One tracked peer: its agent-card name and when we last acted on it.
struct PeerEntry {
    card_name: String,
    last_seen: Instant,
}

/// Liveness map keyed by mDNS instance name.
#[derive(Default)]
pub struct PeerLiveness {
    peers: HashMap<String, PeerEntry>,
}

impl PeerLiveness {
    /// Create an empty tracker.
    pub fn new() -> Self {
        Self {
            peers: HashMap::new(),
        }
    }

    /// Whether `instance` was acted on within `refresh` (skip the duplicate).
    pub fn recently_seen(&self, instance: &str, refresh: Duration) -> bool {
        self.peers
            .get(instance)
            .is_some_and(|e| e.last_seen.elapsed() < refresh)
    }

    /// Record a sighting; returns `true` on the first sight of `card_name`.
    pub fn record(&mut self, instance: &str, card_name: &str) -> bool {
        let first = !self.peers.values().any(|e| e.card_name == card_name);
        self.peers.insert(
            instance.to_string(),
            PeerEntry {
                card_name: card_name.to_string(),
                last_seen: Instant::now(),
            },
        );
        first
    }

    /// Remove peers not seen within `ttl`; returns their card names.
    ///
    /// A multi-homed peer may be tracked under several instances; a
    /// card name is only expired when **all** of its instances have gone
    /// silent, so we don't prematurely deregister a healthy peer.
    pub fn expire(&mut self, ttl: Duration) -> Vec<String> {
        let mut expired_candidates = Vec::new();
        self.peers.retain(|_, e| {
            let alive = e.last_seen.elapsed() < ttl;
            if !alive {
                expired_candidates.push(e.card_name.clone());
            }
            alive
        });
        // Only report a card name as expired if no remaining instance
        // still tracks it (handles multi-homed peers).
        expired_candidates.retain(|cn| !self.peers.values().any(|e| e.card_name == *cn));
        expired_candidates.sort();
        expired_candidates.dedup();
        expired_candidates
    }
}
