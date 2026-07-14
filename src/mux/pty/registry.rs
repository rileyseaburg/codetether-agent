//! Window-to-PTY process ownership for one mux server.

use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};

use anyhow::{Result, bail};

use super::{TerminalSize, program::PtyProgram};

pub(in crate::mux) struct PtyRegistry {
    pub(super) programs: Mutex<HashMap<u64, Arc<PtyProgram>>>,
}

impl PtyRegistry {
    pub(in crate::mux) fn new() -> Self {
        Self {
            programs: Mutex::new(HashMap::new()),
        }
    }

    pub(in crate::mux) fn start(
        &self,
        id: u64,
        command: &str,
        workspace: &Path,
        size: TerminalSize,
    ) -> Result<u64> {
        let mut programs = self.programs.lock().unwrap();
        if programs.get(&id).is_some_and(|program| program.running()) {
            bail!("window {id} already has a running program");
        }
        let program = Arc::new(PtyProgram::start(command, workspace, size)?);
        let offset = program.earliest();
        programs.insert(id, program);
        Ok(offset)
    }

    pub(in crate::mux) fn attach(&self, id: u64, size: TerminalSize) -> Result<u64> {
        let program = self.get(id)?;
        if program.running() {
            program.resize(size)?;
        }
        Ok(program.earliest())
    }
}
