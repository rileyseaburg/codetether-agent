//! Move bridge resources between registration-typed and plain handles.
use super::*;
impl<R: Registration> From<RegisteredBridge<R>> for Bridge {
    fn from(bridge: RegisteredBridge<R>) -> Self {
        Self {
            worker_id: bridge.worker_id,
            worker_name: bridge.worker_name,
            cmd_tx: bridge.cmd_tx,
            task_rx: bridge.task_rx,
            handle: bridge.handle,
        }
    }
}
impl<R: Registration> From<Bridge> for RegisteredBridge<R> {
    fn from(bridge: Bridge) -> Self {
        Self {
            worker_id: bridge.worker_id,
            worker_name: bridge.worker_name,
            cmd_tx: bridge.cmd_tx,
            task_rx: bridge.task_rx,
            handle: bridge.handle,
            registration: PhantomData,
        }
    }
}
