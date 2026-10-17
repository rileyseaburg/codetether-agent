//! External Bun/Node REPL process wrapper.
//!
//! [ExternalRepl::spawn_bun] starts a process with the existing context helper
//! script. [ExternalRepl::execute] exchanges line-framed commands and output;
//! [ExternalRepl::destroy] stops the child. The caller controls process lifetime.
use crate::repl::{ReplResult, ReplRuntime};
use anyhow::Result;
mod availability;
mod execute;
mod framing;
mod init;
mod lifecycle;
mod spawn;
mod state;
pub use state::ExternalRepl;
