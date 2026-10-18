//! Interactive swarm command, monitor guidance and background launch lifecycle.
//!
//! Implement [View] for presentation and [Host] for native executor creation.
//! Call [handle_swarm_command] for task/control commands or [open_swarm_monitor]
//! when the command is bare.
mod command;
mod control;
mod host;
mod launch;
mod monitor;
mod spawn;
pub use command::handle_swarm_command;
pub use host::{Host, View};
pub use monitor::open_swarm_monitor;
