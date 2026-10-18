//! Native entry to shared bus ingress policy.
use crate::tui::app::state::App;
pub fn maybe_queue(app: &mut App, envelope: &crate::bus::BusEnvelope) {
    codetether_bus_view::ingress::maybe_queue(&mut super::ingress_host::Native(app), envelope);
}
