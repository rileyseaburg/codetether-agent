//! Existing native bridge handle shape.
use super::{IncomingTask, WorkerBridgeCmd};
use tokio::sync::mpsc;

/// Result of worker bridge initialization
pub struct TuiWorkerBridge {
    /// Worker ID assigned by the server
    pub worker_id: String,
    /// Worker name
    pub worker_name: String,
    /// Sender for commands (register/deregister agents)
    pub cmd_tx: mpsc::Sender<WorkerBridgeCmd>,
    /// Receiver for incoming tasks
    pub task_rx: mpsc::Receiver<IncomingTask>,
    /// Handle for the bridge task (for shutdown)
    #[allow(dead_code)]
    pub handle: tokio::task::JoinHandle<()>,
}
