use crate::worker_server::WorkerServerState;
pub(super) trait Mode: Sync {
    fn server_state(&self) -> Option<&WorkerServerState>;
}
impl Mode for () {
    fn server_state(&self) -> Option<&WorkerServerState> {
        None
    }
}
impl Mode for &WorkerServerState {
    fn server_state(&self) -> Option<&WorkerServerState> {
        Some(self)
    }
}
