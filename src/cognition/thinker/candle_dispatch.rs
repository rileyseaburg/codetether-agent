//! Blocking Candle inference dispatch from async context.

use anyhow::{Context, Result, anyhow};
use std::sync::{Arc, Mutex};

use super::{CandleThinker, ThinkerOutput};

/// Run Candle inference on a blocking worker, refusing to queue behind a
/// request already in flight.
pub(super) async fn think(
    runtime: &Arc<Mutex<CandleThinker>>,
    system_prompt: &str,
    user_prompt: &str,
) -> Result<ThinkerOutput> {
    let runtime = Arc::clone(runtime);
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
        guard.think(&system_prompt, &user_prompt)
    })
    .await
    .context("candle thinker task join failed")?
}
