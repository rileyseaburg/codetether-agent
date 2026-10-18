//! Conversion from A2A activity metadata to the existing TUI live trace.

use super::projection_apply::apply;
use super::types::RemoteTurn;
use codetether_agent_trace::LiveTraceSnapshot;

pub(super) fn trace(turn: &RemoteTurn) -> Option<LiveTraceSnapshot> {
    turn.is_processing.then(|| {
        let mut trace = LiveTraceSnapshot {
            prompt: turn.prompt.clone(),
            activity: Some("Waiting for A2A peer activity…".into()),
            ..Default::default()
        };
        for event in &turn.activity {
            apply(&mut trace, event);
        }
        trace
    })
}
