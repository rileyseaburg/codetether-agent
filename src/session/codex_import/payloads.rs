use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub(crate) struct CodexResponseMessagePayload {
    pub(crate) role: String,
    #[serde(default)]
    pub(crate) content: Vec<CodexContentItem>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexContentItem {
    #[serde(rename = "type")]
    pub(crate) kind: String,
    #[serde(default)]
    pub(crate) text: Option<String>,
    #[serde(default)]
    pub(crate) image_url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexFunctionCallPayload {
    pub(crate) name: String,
    pub(crate) arguments: Value,
    #[serde(default)]
    pub(crate) call_id: Option<String>,
    #[serde(default)]
    pub(crate) id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexFunctionCallOutputPayload {
    pub(crate) call_id: String,
    pub(crate) output: Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexReasoningPayload {
    #[serde(default)]
    pub(crate) summary: Vec<Value>,
    #[serde(default)]
    pub(crate) content: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTokenEnvelope {
    pub(crate) info: CodexTokenInfo,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTokenInfo {
    pub(crate) total_token_usage: CodexTotalTokenUsage,
}

#[derive(Debug, Deserialize)]
pub(crate) struct CodexTotalTokenUsage {
    #[serde(default)]
    pub(crate) input_tokens: usize,
    #[serde(default)]
    pub(crate) cached_input_tokens: Option<usize>,
    #[serde(default)]
    pub(crate) output_tokens: usize,
    #[serde(default)]
    pub(crate) total_tokens: usize,
}
