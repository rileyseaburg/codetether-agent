//! Native storage binding for shared mux-agent output reading.
use anyhow::Result;
struct ProjectionReader;
impl codetether_mux_control::AgentProjectionReader for ProjectionReader {
    async fn read(&self, id: &str, name: &str) -> Result<Option<serde_json::Value>> {
        codetether_session_store::projection::read_session(id, name).await
    }
}
pub async fn read_agent_output(name: &str) -> Result<Option<String>> {
    codetether_mux_control::read_agent_output(name, &ProjectionReader).await
}
