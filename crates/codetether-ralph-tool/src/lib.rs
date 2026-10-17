//! PRD-driven Ralph tool lifecycle, cleanup and result reporting.
//!
//! Construct [RalphTool] with a [Host] that executes the application Ralph
//! loop and preserves its checkout policy. The crate handles PRD actions,
//! run configuration, cleanup, status messages and result metadata.

mod branch;
mod config;
mod create;
mod dispatch;
mod host;
mod metadata;
mod next_steps;
mod notes;
mod params;
mod render;
mod restore;
mod run;
mod schema;
mod status;
mod tool;
mod tool_impl;
pub use host::Host;
pub use tool::RalphTool;
