//! Read-only retained-output discovery without resizing the live PTY.

use anyhow::Result;

use super::{PtyAttach, registry::PtyRegistry};

impl PtyRegistry {
    pub(in crate::mux) fn tail(&self, id: u64) -> Result<PtyAttach> {
        Ok(self.get(id)?.attach_state())
    }
}
