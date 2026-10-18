//! Top-provider response struct.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TopProvider {
    pub context_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<usize>,
    pub is_moderated: bool,
}
