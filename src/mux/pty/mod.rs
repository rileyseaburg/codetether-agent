//! Server-owned pseudo-terminal processes for detachable mux programs.

mod buffer;
mod monitor;
mod program;
mod program_io;
mod reader;
mod registry;
mod registry_io;
mod resize;
mod spawn;
mod types;

pub(super) use registry::PtyRegistry;
pub(super) use types::{PtyChunk, TerminalSize};
