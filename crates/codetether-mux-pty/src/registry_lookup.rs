//! Checked program lookup for registry operations.

use super::{program::PtyProgram, registry::PtyRegistry};
use anyhow::Result;
use std::sync::Arc;

impl PtyRegistry {
    pub(super) fn live(&self, id: u64) -> Result<Arc<PtyProgram>> {
        let program = self.get(id)?;
        if program.running() {
            Ok(program)
        } else {
            Err(missing(id))
        }
    }

    pub(super) fn get(&self, id: u64) -> Result<Arc<PtyProgram>> {
        self.programs
            .lock()
            .unwrap()
            .get(&id)
            .cloned()
            .ok_or_else(|| missing(id))
    }
}

fn missing(id: u64) -> anyhow::Error {
    anyhow::anyhow!("window {id} has no running program")
}
