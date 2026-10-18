//! Native session and frontend adapters for detached background snapshots.
pub mod child;
pub mod command;
mod host;
pub use command::handle_detach_command;
