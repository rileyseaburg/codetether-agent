//! Native worker startup after its context has been initialized.
//!
//! [bootstrap_worker] delegates sequencing and failure logging to worker-core,
//! using native registration, synchronization and pending-task transport.
use crate::{WorkerContext, pending::Dispatcher};
use std::marker::PhantomData;
mod host;
/// Register, synchronize and fetch pending tasks before scheduling recurring sync.
/// # Arguments
/// * `context` - Fully initialized worker state.
/// # Returns
/// Success after attempting all startup services.
/// # Errors
/// Retains worker-core's behavior: individual startup failures are logged and
/// later steps are still attempted.
pub async fn bootstrap_worker<D: Dispatcher>(context: &WorkerContext) -> anyhow::Result<()> {
    codetether_a2a_worker_core::initialization::bootstrap::bootstrap_worker(&host::Native::<D> {
        context,
        dispatcher: PhantomData,
    })
    .await
}
