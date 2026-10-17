//! Wire representation of Choice.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct Choice {
    pub(crate) message: ChoiceMessage,
    #[serde(default)]
    pub(crate) finish_reason: Option<String>,
}
