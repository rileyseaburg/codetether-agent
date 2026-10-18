//! Values shared by the text and JSON report projections.
pub(super) struct Output<'a> {
    pub args: &'a codetether_cli_options::RlmArgs,
    pub runs: &'a [crate::RunRecord],
    pub provider_name: &'a str,
    pub model: &'a str,
    pub content_type: &'a codetether_rlm::ContentType,
    pub input_tokens: usize,
    pub result: &'a codetether_rlm::RlmAnalysisResult,
    pub oracle_status: &'a str,
    pub oracle_json: &'a serde_json::Value,
}
