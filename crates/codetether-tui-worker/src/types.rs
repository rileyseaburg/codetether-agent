//! Bridge commands, channels and owned task handles.
use codetether_worker_events::IncomingTask;
use tokio::sync::mpsc;

/// Command sent to the worker bridge to register/deregister sub-agents.
/// # Examples
/// ```
/// assert!(matches!(codetether_tui_worker::WorkerBridgeCmd::Shutdown, codetether_tui_worker::WorkerBridgeCmd::Shutdown));
/// ```
#[derive(Debug, Clone)]
pub enum WorkerBridgeCmd {
    /// Register a sub-agent with the A2A server
    RegisterAgent { name: String, instructions: String },
    /// Deregister a sub-agent
    DeregisterAgent { name: String },
    /// Update the processing status (for heartbeat)
    SetProcessing(bool),
    /// Stop the bridge and its heartbeat/SSE child tasks.
    Shutdown,
}

/// Result of worker bridge initialization.
/// # Examples
/// ```text
/// A successful spawn returns a worker identity, command sender, task receiver
/// and task handle for orderly shutdown.
/// ```
pub struct Bridge {
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
