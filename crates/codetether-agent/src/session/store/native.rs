//! Native persistence access to the configured history sink.
impl codetether_native_session_store::Session for crate::session::Session {
    fn history_sink(&self) -> Option<codetether_history_sink::HistorySinkConfig> {
        self.metadata.history_sink.clone()
    }
}
