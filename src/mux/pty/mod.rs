//! Server-owned pseudo-terminal processes for detachable mux programs.

#[cfg(all(test, target_os = "linux"))]
mod benchmark;
mod buffer;
mod monitor;
mod program;
mod program_io;
mod program_wait;
mod reader;
mod registry;
mod registry_io;
mod resize;
mod spawn;
pub(super) mod terminal_mode;
mod types;

pub(super) use registry::PtyRegistry;
pub(super) use types::{PtyAttach, PtyChunk, TerminalSize};
