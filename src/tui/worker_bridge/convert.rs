//! Field-preserving ownership conversion between native and shared bridges.
use super::TuiWorkerBridge;
use codetether_tui_worker::Bridge;
impl From<TuiWorkerBridge> for Bridge {
    fn from(bridge: TuiWorkerBridge) -> Self {
        Self {
            worker_id: bridge.worker_id,
            worker_name: bridge.worker_name,
            cmd_tx: bridge.cmd_tx,
            task_rx: bridge.task_rx,
            handle: bridge.handle,
        }
    }
}
impl From<Bridge> for TuiWorkerBridge {
    fn from(bridge: Bridge) -> Self {
        Self {
            worker_id: bridge.worker_id,
            worker_name: bridge.worker_name,
            cmd_tx: bridge.cmd_tx,
            task_rx: bridge.task_rx,
            handle: bridge.handle,
        }
    }
}
