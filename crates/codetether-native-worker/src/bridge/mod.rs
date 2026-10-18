//! Native worker registration binding for the shared TUI bridge runtime.
mod registration;
/// Worker handle bound to the application's canonical registration metadata.
pub type TuiWorkerBridge =
    codetether_tui_worker::RegisteredBridge<registration::NativeRegistration>;
pub use codetether_tui_worker::{IncomingTask, WorkerBridgeCmd};
