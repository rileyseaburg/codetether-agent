//! Private schema for structured thinker output.
use serde::Deserialize;

/// Raw claim extracted from LLM structured output.
#[derive(Debug, Clone, Deserialize)]
pub(super) struct ExtractedClaim {
    pub(super) claim: String,
    pub(super) belief_key: String,
    pub(super) confidence: f32,
    #[serde(default)]
    pub(super) evidence_refs: Vec<String>,
    #[serde(default)]
    pub(super) contest_target: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    uncertainties: Vec<String>,
}

/// Wrapper for the structured extraction response.
#[derive(Debug, Deserialize)]
pub(super) struct ExtractionResponse {
    pub(super) claims: Vec<ExtractedClaim>,
}
