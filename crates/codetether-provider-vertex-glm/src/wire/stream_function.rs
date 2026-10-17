//! Wire representation of StreamFunction.
use crate::*;
#[derive(Debug, Deserialize)]
pub(crate) struct StreamFunction {
    #[serde(default)]
    pub(crate) name: Option<String>,
    #[serde(default)]
    pub(crate) arguments: Option<String>,
}
