//! Platform-specific terminal attachment for persistent commands.
//!
//! [attach] configures child stdio before spawning and returns an [Attached]
//! transport: a Unix PTY master or the non-Unix pipe fallback.

#[cfg(unix)]
use std::fs::File;
use std::io;

/// The input/output transport attached to a spawned interactive command.
///
/// Variants depend on the target platform.
///
/// # Examples
/// ```rust,no_run
/// let mut command = tokio::process::Command::new("example");
/// let attached = codetether_command_pty::attach(&mut command)?;
/// #[cfg(unix)]
/// let codetether_command_pty::Attached::Pty(_master) = attached;
/// #[cfg(not(unix))]
/// let codetether_command_pty::Attached::Pipes = attached;
/// # Ok::<(), std::io::Error>(())
/// ```
pub enum Attached {
    /// A real pseudo-terminal master controlling the child slave.
    #[cfg(unix)]
    Pty(File),
    /// Pipe fallback on platforms without the Unix PTY implementation.
    #[cfg(not(unix))]
    Pipes,
}

#[cfg(unix)]
#[path = "unix.rs"]
mod platform;
#[cfg(not(unix))]
#[path = "fallback.rs"]
mod platform;

/// Configure an interactive terminal before spawning `command`.
///
/// # Arguments
/// * `command` - Child command whose standard streams should use the transport.
///
/// # Returns
/// The parent transport to retain while communicating with the child.
///
/// # Errors
/// Returns the OS error if terminal allocation or descriptor duplication fails.
///
/// # Examples
/// ```rust,no_run
/// let mut command = tokio::process::Command::new("example");
/// let _transport = codetether_command_pty::attach(&mut command)?;
/// # Ok::<(), std::io::Error>(())
/// ```
pub fn attach(command: &mut tokio::process::Command) -> io::Result<Attached> {
    platform::attach(command)
}
