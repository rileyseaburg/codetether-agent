//! Runtime-handle control contract for the shared active controller.
use super::ActiveCancel;
use codetether_session_steering::SteeringInput;

impl codetether_session_runtime::handle::Control for ActiveCancel {
    type Input = SteeringInput;
    fn prepare(&self, session_id: &str) -> bool {
        ActiveCancel::prepare(self, session_id)
    }
    fn notify(&self) -> bool {
        ActiveCancel::notify(self)
    }
    fn clear(&self) {
        ActiveCancel::clear(self);
    }
    fn steer(&self, input: SteeringInput) -> bool {
        ActiveCancel::steer(self, input)
    }
}
