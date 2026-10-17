//! Wire representation of FunctionCall.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct FunctionCall {
    pub(crate) name: String,
    pub(crate) arguments: String,
}
