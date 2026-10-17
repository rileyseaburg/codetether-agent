//! LoggingMessageParams wire contract and construction.
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingMessageParams {
    pub level: LogLevel,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    pub data: Value,
}
