//! Runtime operations on PTY programs already owned by the registry.

use std::sync::Arc;

use anyhow::Result;

use super::{PtyChunk, TerminalSize, program::PtyProgram, registry::PtyRegistry};

impl PtyRegistry {
    pub(in crate::mux) fn input(&self, id: u64, data: &[u8]) -> Result<()> {
        self.live(id)?.input(data)
    }

    pub(in crate::mux) async fn read(&self, id: u64, offset: u64) -> Result<PtyChunk> {
        Ok(self.get(id)?.read_wait(offset).await)
    }

    pub(in crate::mux) fn resize(&self, id: u64, size: TerminalSize) -> Result<()> {
        self.live(id)?.resize(size)
    }

    pub(in crate::mux) fn stop(&self, id: u64) {
        if let Some(program) = self.programs.lock().unwrap().remove(&id) {
            program.stop();
        }
    }

    pub(in crate::mux) fn stop_all(&self) {
        for program in self.programs.lock().unwrap().values() {
            program.stop();
        }
    }

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
