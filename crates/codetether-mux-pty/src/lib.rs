//! Server-owned pseudo-terminal processes for detachable mux programs.
//!
//! [PtyRegistry] owns window programs and provides input, retained-output reads,
//! attachment and resize operations. Construct it with [PtyRegistry::new], then
//! start a program using a workspace, [TerminalSize] and inherited mux identity.
//! [default_shell] chooses a platform login shell.

#[cfg(all(test, target_os = "linux"))]
mod benchmark;
mod buffer;
pub mod default_shell;
mod monitor;
mod program;
mod program_io;
mod program_wait;
mod reader;
mod registry;
mod registry_io;
mod registry_tail;
mod resize;
mod spawn;

mod program_lifecycle;
mod registry_lookup;
mod types;

pub use registry::PtyRegistry;
pub use types::{PtyAttach, PtyChunk, TerminalSize};
