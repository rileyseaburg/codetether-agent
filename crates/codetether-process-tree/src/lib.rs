//! Cancellation-safe ownership of tool subprocess trees.
//!
//! Call [configure] before spawning a command, then attach [Guard] to its child.
//! Dropping an armed guard terminates descendants; [Guard::disarm] ends tracking.

use tokio::process::{Child, Command};

#[cfg(not(any(unix, windows)))]
#[path = "fallback.rs"]
mod platform;
#[cfg(unix)]
#[path = "unix.rs"]
mod platform;
#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

#[cfg(test)]
#[path = "tests.rs"]
mod tests;

/// Configures a command so dropping its owner stops the immediate child.
///
/// # Arguments
/// * `command` - Command to configure before spawning.
///
/// # Returns
/// Updates the command in place.
///
/// # Examples
/// ```rust
/// let mut command = tokio::process::Command::new("example");
/// codetether_process_tree::configure(&mut command);
/// ```
pub fn configure(command: &mut Command) {
    command.kill_on_drop(true);
    platform::configure(command);
}

/// Kills a configured command's descendants unless normal exit disarms it.
///
/// # Examples
/// ```rust,no_run
/// # fn track(child: &tokio::process::Child) {
/// let mut guard = codetether_process_tree::Guard::attach(child);
/// // After the command completes normally:
/// guard.disarm();
/// # }
/// ```
pub struct Guard {
    platform: platform::Guard,
    armed: bool,
}

impl Guard {
    /// Starts tracking the process tree rooted at `child`.
    pub fn attach(child: &Child) -> Self {
        Self {
            platform: platform::Guard::attach(child.id()),
            armed: true,
        }
    }

    /// Preserves surviving descendants after a normal command exit.
    pub fn disarm(&mut self) {
        if !self.armed {
            return;
        }
        self.platform.disarm();
        self.armed = false;
    }
}

impl Drop for Guard {
    /// Terminates the owned process tree when its execution future is dropped.
    fn drop(&mut self) {
        if self.armed {
            self.platform.terminate();
        }
    }
}
