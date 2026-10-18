//! Native entry to shared bus ingress policy.
use crate::tui::app::state::App;
pub fn track(app: &mut App, message: &crate::bus::BusMessage) {
    codetether_bus_view::ingress::track(&mut super::ingress_host::Native(app), message);
}
