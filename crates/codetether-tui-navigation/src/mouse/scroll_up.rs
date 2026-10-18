//! View-mode-aware scrolling through existing navigation actions.
use super::Host;
use crate::Action;
use codetether_tui_models::ViewMode;
pub(super) fn scroll_up_by_mode(host: &mut impl Host, amount: usize) {
    let context = host.context();
    match context.view_mode {
        ViewMode::Chat => host.apply(Action::ChatUp(amount)),
        ViewMode::Swarm => {
            if context.swarm_detail {
                host.apply(Action::SwarmDetailUp(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::SwarmPrevious);
                }
            }
        }
        ViewMode::Ralph => {
            if context.ralph_detail {
                host.apply(Action::RalphDetailUp(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::RalphPrevious);
                }
            }
        }
        ViewMode::Bus if !context.bus_filter => {
            if context.bus_detail {
                host.apply(Action::BusDetailUp(amount));
            } else {
                for _ in 0..amount {
                    host.apply(Action::BusPrevious);
                }
            }
        }
        ViewMode::Audit => {
            for _ in 0..amount {
                host.audit_previous();
            }
        }
        _ => {}
    }
}
