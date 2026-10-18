//! Remove focus only when no local, owned-tool or swarm agent resolves.
use super::{Host, swarm};
pub(super) fn discard_stale_focus(app: &mut impl Host) {
    let Some(name) = app.selected() else {
        return;
    };
    let local = app.local_contains(name);
    let swarm = swarm::contains(app, name);
    let owned_tool = app.owned_tool_contains(name);
    if !local && !owned_tool && !swarm {
        app.clear_selected();
    }
}
