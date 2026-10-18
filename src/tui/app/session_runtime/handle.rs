//! Native specialization of the shared runtime command handle.
use super::{PromptRequest, active_cancel::ActiveCancel};
pub(crate) type TuiSessionHandle =
    codetether_session_runtime::handle::SessionHandle<PromptRequest, ActiveCancel>;
