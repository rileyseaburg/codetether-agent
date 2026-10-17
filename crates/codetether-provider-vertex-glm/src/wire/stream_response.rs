//! Wire representation of StreamResponse.
use crate::*;
// SSE streaming types
#[derive(Debug, Deserialize)]
pub(crate) struct StreamResponse {
    pub(crate) choices: Vec<StreamChoice>,
}
