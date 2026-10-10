//! Registry plus runtimes, mutated only behind the relay's single mutex.
use crate::{ApiError, runtime::Runtime};
use codetether_companion_core::{Error, Registry};
use std::collections::HashMap;

/// Serialized relay state: core sessions and their analysis runtimes.
#[derive(Default)]
pub(crate) struct State {
    pub registry: Registry,
    pub runtimes: HashMap<String, Runtime>,
}
impl State {
    /// Stop and drop runtimes whose core session ended, then sweep the registry.
    pub(crate) fn sweep(&mut self, now: i64) {
        let registry = &mut self.registry;
        let ended: Vec<String> = self
            .runtimes
            .keys()
            .filter(|id| registry.session(id, now).is_err())
            .cloned()
            .collect();
        for id in ended {
            if let Some(mut runtime) = self.runtimes.remove(&id) {
                runtime.stop();
            }
        }
        self.registry.sweep(now);
    }
    /// Live runtime for `id`; an expired session is stopped on access.
    pub(crate) fn live(&mut self, id: &str, now: i64) -> Result<&mut Runtime, ApiError> {
        if let Err(error) = self.registry.session(id, now) {
            if error == Error::Ended
                && let Some(runtime) = self.runtimes.get_mut(id)
            {
                runtime.stop();
            }
            return Err(error.into());
        }
        self.runtimes
            .get_mut(id)
            .ok_or_else(|| Error::NotFound.into())
    }
}
