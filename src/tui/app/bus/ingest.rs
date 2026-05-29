use crate::bus::BusHandle;
use crate::tui::app::state::App;

pub fn drain(app: &mut App, bus_handle: &mut BusHandle) {
    while let Some(envelope) = bus_handle.try_recv() {
        super::agent_track::track(app, &envelope.message);
        super::inbox::maybe_queue(app, &envelope);
        app.state.bus_log.ingest(&envelope);
    }
}
