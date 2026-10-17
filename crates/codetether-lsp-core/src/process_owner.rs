//! Own the isolated language-server process group, not just its launcher.

use std::ops::{Deref, DerefMut};
use tokio::process::Child;

/// Kills an isolated Unix process group when its transport is released.
pub(crate) struct ServerProcess(pub(crate) Child);

impl Deref for ServerProcess {
    type Target = Child;

    fn deref(&self) -> &Child {
        &self.0
    }
}

impl DerefMut for ServerProcess {
    fn deref_mut(&mut self) -> &mut Child {
        &mut self.0
    }
}

impl Drop for ServerProcess {
    fn drop(&mut self) {
        #[cfg(unix)]
        if let Some(pid) = self.0.id() {
            // spawn() placed this still-owned child in a new process group.
            // The unreaped child keeps this group ID from being reused.
            unsafe {
                libc::kill(-(pid as libc::pid_t), libc::SIGKILL);
            }
        }
        // kill_on_drop remains the direct-child fallback on other platforms.
    }
}
