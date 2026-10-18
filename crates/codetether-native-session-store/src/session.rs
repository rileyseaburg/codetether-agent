//! Archive configuration on an existing persisted session.
use codetether_history_sink::HistorySinkConfig;
/// Native persistence, indexing and archive source contracts.
/// # Implementors
/// Implement on the existing session type; retain its configured history sink.
pub trait Session:
    codetether_native_recall::Session
    + codetether_native_proactive::Session
    + codetether_session_store::Snapshot<Tool: serde::de::DeserializeOwned>
{
    /// Clone the configured history sink when archival is enabled.
    fn history_sink(&self) -> Option<HistorySinkConfig>;
}
