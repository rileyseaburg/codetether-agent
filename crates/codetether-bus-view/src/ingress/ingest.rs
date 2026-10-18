use super::Host;
use codetether_bus_core::recorder::BusRecorder;

/// Drain new bus envelopes into the TUI views.
///
/// Reads from the bus **recorder** via a monotonic cursor rather than the live
/// `broadcast` receiver, so high-volume traffic (e.g. discovery heartbeats)
/// can no longer silently evict sub-agent tool events before the TUI sees
/// them. On the first call the cursor jumps to the current tail (we only show
/// activity from when the view started); thereafter every recorded envelope is
/// delivered exactly once.
/// Apply recorded bus input through frontend bindings.
/// # Arguments
/// * `host` — Frontend identities, observers and presentation effects.
/// * Input — Recorded envelope, message or recorder to process.
/// # Returns
/// Updates only the state selected by the bus message and routing rules.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_bus_view::ingress::Host, recorder: &codetether_bus_core::recorder::BusRecorder) {
/// codetether_bus_view::ingress::drain(host, recorder);
/// # }
/// ```
pub fn drain(host: &mut impl Host, recorder: &BusRecorder) {
    if *host.cursor() == 0 {
        *host.cursor() = recorder.cursor();
        return;
    }
    let (envelopes, next) = recorder.drain_since(*host.cursor());
    *host.cursor() = next;
    for envelope in &envelopes {
        super::agent_track::track(host, &envelope.message);
        super::inbox::maybe_queue(host, envelope);
        host.observe_tools(&envelope.message);
        host.ingest_log(envelope);
    }
}
