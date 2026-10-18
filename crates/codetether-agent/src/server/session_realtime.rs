//! Application loading and metadata access for native realtime execution.
use crate::session::{Session, SessionMetadata};
impl codetether_native_server::RealtimeSession for Session {
    async fn load_realtime(id: &str) -> anyhow::Result<Self> {
        Session::load(id).await
    }
    fn realtime_metadata(&mut self) -> &mut SessionMetadata {
        &mut self.metadata
    }
}
