//! linux for seccomp filter preparation.
#[path = "bpf.rs"]
mod bpf;
#[path = "fd.rs"]
mod fd;

use anyhow::{Context, Result};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};
use std::os::fd::{AsRawFd, RawFd};

/// Owns the filter descriptor on supported hosts.
///
/// # Examples
/// ```rust,no_run
/// let _program = codetether_sandbox_seccomp::prepare(false)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug)]
pub struct Program {
    file: File,
}

impl Program {
    /// Borrow the descriptor number without releasing ownership.
    pub fn fd(&self) -> RawFd {
        self.file.as_raw_fd()
    }
}

/// Prepare a profile, or return `None` when the platform is unsupported.
///
/// # Arguments
/// * `allow_network` - Whether socket syscalls are omitted from the deny list.
/// # Returns
/// An owned profile descriptor when supported.
/// # Errors
/// Returns profile-file creation, writing, seeking or descriptor errors.
/// # Examples
/// ```rust,no_run
/// let _profile = codetether_sandbox_seccomp::prepare(false)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn prepare(allow_network: bool) -> Result<Option<Program>> {
    let mut file = tempfile::tempfile().context("create seccomp profile file")?;
    file.write_all(&bpf::program(!allow_network))
        .context("write seccomp profile")?;
    file.seek(SeekFrom::Start(0))
        .context("rewind seccomp profile")?;
    fd::make_inheritable(file.as_raw_fd())?;
    Ok(Some(Program { file }))
}

#[cfg(test)]
#[path = "linux_tests.rs"]
mod tests;
