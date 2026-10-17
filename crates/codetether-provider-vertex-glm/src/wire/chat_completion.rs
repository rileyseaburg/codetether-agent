//! Wire representation of ChatCompletion.
use crate::*;
// Response types
#[derive(Debug, Deserialize)]
pub(crate) struct ChatCompletion {
    pub(crate) choices: Vec<Choice>,
    #[serde(default)]
    pub(crate) usage: Option<ApiUsage>,
}
