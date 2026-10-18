//! Parent-scoped collection of unseen child notices.
use super::{Host, extract::extract};
use codetether_bus_core::{BusEnvelope, global::global};
use std::collections::HashSet;

pub(super) fn collect(session: &impl Host, seen: &mut HashSet<String>) -> Option<String> {
    let bus = global()?;
    let topic = format!("agent.{}", session.session_id());
    collect_from(bus.recorder.recent(1024, Some(&topic)), session, seen)
}

fn collect_from(
    envelopes: Vec<BusEnvelope>,
    session: &impl Host,
    seen: &mut HashSet<String>,
) -> Option<String> {
    let lines = envelopes
        .into_iter()
        .filter_map(|envelope| extract(envelope, session, seen))
        .collect::<Vec<_>>();
    (!lines.is_empty()).then(|| format!("Sub-agent results:\n{}", lines.join("\n\n")))
}
