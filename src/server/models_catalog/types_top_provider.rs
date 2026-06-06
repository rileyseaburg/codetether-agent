//! Top-provider response struct.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub(crate) struct TopProvider {
    pub(crate) context_length: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_completion_tokens: Option<usize>,
    pub(crate) is_moderated: bool,
}
