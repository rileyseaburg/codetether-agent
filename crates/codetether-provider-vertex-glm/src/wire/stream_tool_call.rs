//! Wire representation of StreamToolCall.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct StreamToolCall {
    #[serde(default)]
    pub(crate) id: Option<String>,
    pub(crate) function: Option<StreamFunction>,
}
