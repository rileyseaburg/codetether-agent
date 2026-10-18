//! Stable object-key layout for complete and chunked session histories.
use crate::HistorySinkConfig;

/// Object-key suffix where the full transcript lives, relative to
/// [`HistorySinkConfig::prefix`].
const HISTORY_OBJECT_NAME: &str = "history.jsonl";
impl HistorySinkConfig {
    /// Object key for the full transcript of `session_id`.
    ///
    /// Example: `history/1234-abcd/history.jsonl`.
    pub fn object_key(&self, session_id: &str) -> String {
        format!("{}{session_id}/{HISTORY_OBJECT_NAME}", self.prefix)
    }
}
