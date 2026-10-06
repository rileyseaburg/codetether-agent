//! Blocking Candle inference dispatch from async context.

use anyhow::{Context, Result, anyhow};
use std::sync::{Arc, Mutex};

use super::{CandleThinker, ThinkerOutput};

#[path = "candle_dispatch/identity.rs"]
mod identity;

/// Run Candle inference on a blocking worker, refusing to queue behind a
/// request already in flight.
///
/// # Errors
/// Returns busy, poisoned-mutex, inference, or blocking-task failures.
pub(super) async fn think(
    runtime: &Arc<Mutex<CandleThinker>>,
    provider: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let runtime = Arc::clone(runtime);
    let provider = provider.to_owned();
    let system_prompt = system_prompt.to_string();
    let user_prompt = user_prompt.to_string();
    tokio::task::spawn_blocking(move || {
        let mut guard = match runtime.try_lock() {
            Ok(guard) => guard,
            Err(std::sync::TryLockError::WouldBlock) => {
                return Err(anyhow!("candle thinker is busy"));
            }
            Err(std::sync::TryLockError::Poisoned(_)) => {
                return Err(anyhow!("candle thinker mutex poisoned"));
            }
        };
        #[cfg(feature = "candle")]
        let loaded_model = guard.identity.model();
        #[cfg(not(feature = "candle"))]
        let loaded_model = ""; // The disabled runtime always rejects inference.
        let system = identity::system_prompt(&system_prompt, &provider, loaded_model);
        guard.think(&system, &user_prompt)
    })
    .await
    .context("candle thinker task join failed")?
}
