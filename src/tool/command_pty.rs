//! Platform-specific terminal attachment for persistent commands.

use std::io;
#[cfg(unix)]
use std::fs::File;

/// The input/output transport attached to a spawned interactive command.
pub(crate) enum Attached {
    /// A real pseudo-terminal master controlling the child slave.
    #[cfg(unix)]
    Pty(File),
    /// Pipe fallback on platforms without the Unix PTY implementation.
    #[cfg(not(unix))]
    Pipes,
}

#[cfg(unix)]
#[path = "command_pty/unix.rs"]
mod platform;
#[cfg(not(unix))]
#[path = "command_pty/fallback.rs"]
mod platform;

/// Configure an interactive terminal before spawning `command`.
pub(crate) fn attach(command: &mut tokio::process::Command) -> io::Result<Attached> {
    platform::attach(command)
}
