//! Native session-store adapter for bounded history page reads.
use crate::provider::Message;
use crate::session::Session;
use async_trait::async_trait;
pub(super) struct NativeSource;
#[async_trait]
impl codetether_history_page::HistorySource for NativeSource {
    async fn load_tail(id: &str, window: usize) -> Result<Vec<Message>, String> {
        Session::load_tail(id, window)
            .await
            .map(|loaded| loaded.session.messages)
            .map_err(|error| error.to_string())
    }
    async fn read_messages(id: &str, start: usize, limit: usize) -> Result<Vec<Message>, String> {
        Session::read_messages(id, start, limit)
            .await
            .map_err(|error| error.to_string())
    }
}
