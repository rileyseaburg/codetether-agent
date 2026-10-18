//! Swarm header title presentation.
use crate::SwarmViewState;
pub(super) fn title(state: &SwarmViewState) -> &'static str {
    let title = if state.complete {
        if state.error.is_some() {
            " Swarm [ERROR] "
        } else {
            " Swarm [COMPLETE] "
        }
    } else if state.active {
        " Swarm [ACTIVE] "
    } else {
        " Swarm [IDLE] "
    };

    title
}
