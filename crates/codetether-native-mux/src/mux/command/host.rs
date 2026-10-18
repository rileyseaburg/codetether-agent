//! Native services required by shared mux CLI execution.
pub(super) struct Host;
#[async_trait::async_trait]
impl codetether_mux_cli::Host for Host {
    async fn recorded_workspace(&self, session: &str) -> anyhow::Result<std::path::PathBuf> {
        let path = codetether_session_store::lifecycle::session_path(session)?;
        codetether_session_store::metadata::workspace::recorded_workspace(&path, session).await
    }
    fn clipboard_content(&self) -> Option<String> {
        crate::mux::client::clipboard_content()
    }
}
