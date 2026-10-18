//! Native specialization of the shared runtime command handle.
use super::{PromptRequest, active_cancel::ActiveCancel};
use crate::session::helper::steering::SteeringInput;
pub(crate) type TuiSessionHandle =
    codetether_session_runtime::handle::SessionHandle<PromptRequest, ActiveCancel>;
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
