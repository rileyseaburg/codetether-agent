//! Bridge handles associated with their worker-registration implementation.
//!
//! [RegisteredBridge] preserves the owned channels and task handle of [Bridge].
//! Its registration type selects startup metadata without application-owned
//! handle definitions. Convert into [Bridge] for shared shutdown operations.
use crate::{Bridge, IncomingTask, Registration, WorkerBridgeCmd};
use std::marker::PhantomData;
use tokio::sync::mpsc;
mod convert;
mod spawn;

/// Worker bridge carrying its registration policy at the type level.
/// # Examples
/// ```no_run
/// # fn example<R: codetether_tui_worker::Registration>(bridge: codetether_tui_worker::Bridge) {
/// let bridge = codetether_tui_worker::RegisteredBridge::<R>::from(bridge);
/// let bridge: codetether_tui_worker::Bridge = bridge.into();
/// # let _ = bridge;
/// # }
/// ```
pub struct RegisteredBridge<R: Registration> {
    /// Worker ID assigned by the server.
    pub worker_id: String,
    /// Worker name.
    pub worker_name: String,
    /// Sender for worker commands.
    pub cmd_tx: mpsc::Sender<WorkerBridgeCmd>,
    /// Receiver for incoming tasks.
    pub task_rx: mpsc::Receiver<IncomingTask>,
    /// Bridge task used for orderly shutdown.
    pub handle: tokio::task::JoinHandle<()>,
    registration: PhantomData<fn() -> R>,
}
