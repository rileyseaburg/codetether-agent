//! Wire representation of StreamChoice.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct StreamChoice {
    pub(crate) delta: StreamDelta,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
