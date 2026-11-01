//! Share one gate across provider instances and aliases targeting the same origin.
#[cfg(test)]
mod tests;
use super::{config::Config, gate::Gate};
use anyhow::{Result, anyhow, ensure};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock, Weak},
};

static GATES: OnceLock<Mutex<HashMap<String, Weak<Gate>>>> = OnceLock::new();

pub(super) fn gate(base: &str, config: Config) -> Result<Arc<Gate>> {
    let url = reqwest::Url::parse(base)?;
    ensure!(
        matches!(url.scheme(), "http" | "https"),
        "LM Studio endpoint must be HTTP(S)"
    );
    let key = url.origin().ascii_serialization();
    let mut gates = GATES
        .get_or_init(Mutex::default)
        .lock()
        .map_err(|_| anyhow!("LM Studio queue registry unavailable"))?;
    gates.retain(|_, gate| gate.strong_count() > 0);
    if let Some(gate) = gates.get(&key).and_then(Weak::upgrade) {
        ensure!(
            gate.config == config,
            "LM Studio queue configuration changed during active work; restart before reconfiguring"
        );
        return Ok(gate);
    }
    let gate = Gate::new(config);
    gates.insert(key, Arc::downgrade(&gate));
    Ok(gate)
}
