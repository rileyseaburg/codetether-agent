//! Native entry to shared bus ingress policy.
use crate::tui::app::state::App;
pub fn drain(app: &mut App, recorder: &crate::bus::recorder::BusRecorder) {
    codetether_bus_view::ingress::drain(&mut super::ingress_host::Native(app), recorder);
}
