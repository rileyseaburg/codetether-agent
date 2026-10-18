//! Interactive Ralph command arguments and background execution.
//!
//! Parse command arguments with [`parse_ralph_run_args`], resolve the PRD with
//! [`resolve_prd_path`] and implement [`Host`] to launch a native loop through
//! [`spawn_ralph_run`]. View updates remain with the interactive client.
mod arguments;
mod host;
mod launch;
mod path;
pub use arguments::parse_ralph_run_args;
pub use host::Host;
pub use launch::spawn_ralph_run;
pub use path::resolve_prd_path;
