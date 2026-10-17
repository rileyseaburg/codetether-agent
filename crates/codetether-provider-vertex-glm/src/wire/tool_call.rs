//! Wire representation of ToolCall.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct ToolCall {
    pub(crate) id: String,
    pub(crate) function: FunctionCall,
}
