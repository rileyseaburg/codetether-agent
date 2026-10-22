//! envelope wire representation.
use super::*;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub(crate) struct DsResponse {
    #[allow(dead_code)]
    pub id: String,
    #[allow(dead_code)]
    pub model: String,
    pub choices: Vec<DsChoice>,
    #[serde(default)]
    pub usage: Option<DsUsage>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct DsChoice {
    pub message: DsMessage,
    #[serde(default)]
    pub finish_reason: Option<String>,
}
