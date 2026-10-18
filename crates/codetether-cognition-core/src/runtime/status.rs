//! Summary status reporting and internal event publishing.

use std::sync::atomic::Ordering;

use super::CognitionRuntime;
use crate::thought::buffers::push_event_internal;
use crate::{ThoughtEvent, persona::PersonaStatus, status::CognitionStatus};

impl<T, R> CognitionRuntime<T, R> {
    /// Return a summary of runtime and persona state.
    pub async fn status(&self) -> CognitionStatus {
        let personas = self.personas.read().await;
        let events = self.events.read().await;
        let snapshots = self.snapshots.read().await;

        let active_persona_count = personas
            .values()
            .filter(|p| p.status == PersonaStatus::Active)
            .count();

        CognitionStatus {
            enabled: self.enabled,
            running: self.running.load(Ordering::SeqCst),
            loop_interval_ms: *self.loop_interval_ms.read().await,
            started_at: *self.started_at.read().await,
            last_tick_at: *self.last_tick_at.read().await,
            persona_count: personas.len(),
            active_persona_count,
            events_buffered: events.len(),
            snapshots_buffered: snapshots.len(),
        }
    }

    /// Append an event to the bounded buffer and broadcast it.
    pub async fn push_event(&self, event: ThoughtEvent) {
        push_event_internal(&self.events, self.max_events, &self.event_tx, event).await;
    }
}
