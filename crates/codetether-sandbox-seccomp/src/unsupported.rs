//! unsupported for seccomp filter preparation.
use anyhow::Result;

/// Owns the filter descriptor on supported hosts.
///
/// # Examples
/// ```rust,no_run
/// let _program = codetether_sandbox_seccomp::prepare(false)?;
/// # Ok::<(), anyhow::Error>(())
/// ```
#[derive(Debug)]
pub struct Program;

impl Program {
    /// Borrow the descriptor number without releasing ownership.
    pub fn fd(&self) -> i32 {
        -1
    }
}

/// Non-Linux commands have no seccomp filter to attach.
pub fn apply(_command: &mut tokio::process::Command, _program: &Program) {}

/// Synchronous non-Linux commands likewise have no seccomp filter.
pub fn apply_std(_command: &mut std::process::Command, _program: &Program) {}

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
pub fn prepare(_allow_network: bool) -> Result<Option<Program>> {
    Ok(None)
}
