//! Window-to-PTY process ownership for one mux server.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::{Result, bail};

use super::{PtyAttach, TerminalSize, program::PtyProgram};

/// Owns the PTY programs associated with one server's window identifiers.
///
/// # Examples
/// ```
/// let registry = codetether_mux_pty::PtyRegistry::new();
/// registry.stop_all();
/// ```
pub struct PtyRegistry {
    pub(super) programs: Mutex<HashMap<u64, Arc<PtyProgram>>>,
}

impl PtyRegistry {
    /// Create an empty program registry.
    pub fn new() -> Self {
        Self {
            programs: Mutex::new(HashMap::new()),
        }
    }

    /// Start a shell command in a window and return its earliest replay offset.
    ///
    /// # Arguments
    /// * `id` - Window identifier owned by this registry.
    /// * `command` - Shell command to execute.
    /// * `workspace` - Child working directory.
    /// * `size` - Initial terminal dimensions.
    /// * `mux_session` - Identity inherited by the child for coordination.
    /// # Returns
    /// The first retained output offset.
    /// # Errors
    /// Returns an error for a live duplicate window or terminal/process creation failure.
    /// # Panics
    /// Panics if a registry mutex is poisoned or the child reaper cannot start.
    /// # Examples
    /// ```no_run
    /// # fn example() -> anyhow::Result<()> {
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// registry.start(1, "echo ready", std::path::Path::new("."),
    ///     codetether_mux_pty::TerminalSize::new(80, 24), "work")?;
    /// registry.stop_all();
    /// # Ok(()) }
    /// ```
    pub fn start(
        &self,
        id: u64,
        command: &str,
        workspace: &Path,
        size: TerminalSize,
        mux_session: &str,
    ) -> Result<u64> {
        let mut programs = self.programs.lock().unwrap();
        if programs.get(&id).is_some_and(|program| program.running()) {
            bail!("window {id} already has a running program");
        }
        let program = Arc::new(PtyProgram::start(command, workspace, size, mux_session)?);
        let offset = program.earliest();
        programs.insert(id, program);
        Ok(offset)
    }

    /// Capture replay state and resize a running window before attachment.
    ///
    /// # Arguments
    /// * `id` - Window identifier.
    /// * `size` - Requested terminal dimensions.
    /// # Returns
    /// The retained replay boundaries and screen mode.
    /// # Errors
    /// Returns an error if the window is absent or resizing fails.
    /// # Panics
    /// Panics if an internal mutex is poisoned.
    /// # Examples
    /// ```
    /// let registry = codetether_mux_pty::PtyRegistry::new();
    /// assert!(registry.attach(1, codetether_mux_pty::TerminalSize::new(80, 24)).is_err());
    /// ```
    pub fn attach(&self, id: u64, size: TerminalSize) -> Result<PtyAttach> {
        let program = self.get(id)?;
        let attach = program.attach_state();
        if program.running() {
            program.resize(size)?;
        }
        Ok(attach)
    }
}
