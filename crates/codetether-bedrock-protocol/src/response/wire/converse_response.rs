//! ConverseResponse wire shape.
use super::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ConverseResponse {
    pub(crate) output: ConverseOutput,
    #[serde(default)]
    pub(crate) stop_reason: Option<String>,
    #[serde(default)]
    pub(crate) usage: Option<ConverseUsage>,
}
