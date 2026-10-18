//! View-mode-aware scrolling through existing navigation actions.
use super::Host;
use crate::Action;
use codetether_tui_models::ViewMode;
pub(super) fn scroll_down_by_mode(host: &mut impl Host, amount: usize) {
    let context = host.context();
    match context.view_mode {
        ViewMode::Chat => host.apply(Action::ChatDown(amount)),
        ViewMode::Swarm => {
            if context.swarm_detail {
                host.apply(Action::SwarmDetailDown(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::SwarmNext);
                }
            }
        }
        ViewMode::Ralph => {
            if context.ralph_detail {
                host.apply(Action::RalphDetailDown(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::RalphNext);
                }
            }
        }
        ViewMode::Bus if !context.bus_filter => {
            if context.bus_detail {
                host.apply(Action::BusDetailDown(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::BusNext);
                }
            }
        }
        ViewMode::Audit => {
            for _ in 0..amount {
                host.audit_next();
            }
        }
        _ => {}
    }
}
