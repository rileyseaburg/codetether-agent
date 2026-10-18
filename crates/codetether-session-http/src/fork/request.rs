//! Bind an edit to the exact source message the client observed.
use serde::Deserialize;

/// Bind the edit to the exact message observed by the client.
#[derive(Deserialize)]
pub(super) struct ForkRequest {
    pub(super) before_message: usize,
    pub(super) expected_text: String,
}
