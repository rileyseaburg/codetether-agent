//! Cancellation-safe ownership of tool subprocess trees.

use tokio::process::{Child, Command};

#[cfg(not(any(unix, windows)))]
#[path = "process_tree/fallback.rs"]
mod platform;
#[cfg(unix)]
#[path = "process_tree/unix.rs"]
mod platform;
#[cfg(windows)]
#[path = "process_tree/windows.rs"]
mod platform;

#[cfg(test)]
#[path = "process_tree/tests.rs"]
mod tests;

/// Configures a command so dropping its owner stops the immediate child.
pub(super) fn configure(command: &mut Command) {
    command.kill_on_drop(true);
    platform::configure(command);
}

/// Kills a configured command's descendants unless normal exit disarms it.
pub(super) struct Guard {
    platform: platform::Guard,
    armed: bool,
}

impl Guard {
    /// Starts tracking the process tree rooted at `child`.
    pub(super) fn attach(child: &Child) -> Self {
        Self {
            platform: platform::Guard::attach(child.id()),
            armed: true,
        }
    }

    /// Preserves surviving descendants after a normal command exit.
    pub(super) fn disarm(&mut self) {
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
