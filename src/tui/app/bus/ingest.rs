use crate::bus::BusHandle;
use crate::tui::app::state::App;

/// Drain new bus envelopes into the TUI views.
///
/// Reads from the bus **recorder** via a monotonic cursor rather than the live
/// `broadcast` receiver, so high-volume traffic (e.g. discovery heartbeats)
/// can no longer silently evict sub-agent tool events before the TUI sees
/// them. On the first call the cursor jumps to the current tail (we only show
/// activity from when the view started); thereafter every recorded envelope is
/// delivered exactly once.
pub fn drain(app: &mut App, bus_handle: &mut BusHandle) {
    let recorder = bus_handle.recorder();
    if app.state.bus_cursor == 0 {
        app.state.bus_cursor = recorder.cursor();
        return;
    }
    let (envelopes, next) = recorder.drain_since(app.state.bus_cursor);
    app.state.bus_cursor = next;
    for envelope in &envelopes {
        super::agent_track::track(app, &envelope.message);
        super::inbox::maybe_queue(app, envelope);
        app.state.tool_calls.observe(&envelope.message);
        app.state.bus_log.ingest(envelope);
    }
}
