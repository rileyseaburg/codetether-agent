//! Native mux services and prompt-lifetime lease integration.
#[path = "workspace_coordination/gate.rs"]
mod gate;
#[path = "workspace_coordination/host.rs"]
mod host;
#[path = "workspace_coordination/turn.rs"]
mod turn;
pub(super) use gate::blocked;
pub(super) use turn::LeaseTurn;
