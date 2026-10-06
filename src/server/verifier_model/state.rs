//! Configuration service state shared with the verifier harness, not an HTTP-only copy.

use super::{defaults::Defaults, types::Settings};
use crate::tool::goal::verify::{
    VerifierSelection,
    observation::{ObservationStore, shared_observations},
    shared_selection,
};
use std::sync::Arc;

#[derive(Clone)]
pub(super) struct ApiState {
    pub selection: Arc<VerifierSelection>,
    pub observations: Arc<ObservationStore>,
    #[cfg(test)]
    pub defaults: Option<Defaults>,
}

impl Default for ApiState {
    fn default() -> Self {
        Self {
            selection: shared_selection(),
            observations: shared_observations(),
            #[cfg(test)]
            defaults: None,
        }
    }
}

impl ApiState {
    pub async fn defaults(&self) -> anyhow::Result<Defaults> {
        #[cfg(test)]
        if let Some(defaults) = &self.defaults {
            return Ok(defaults.clone());
        }
        Defaults::load().await
    }
    pub fn snapshot(&self, defaults: Defaults, worker: Option<String>) -> Settings {
        super::projection::snapshot(
            self.selection.get(),
            defaults,
            worker,
            self.observations.latest(),
        )
    }
}
