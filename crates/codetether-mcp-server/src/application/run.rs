//! Native entry point for the shared MCP receive loop.
use super::{Application, ApplicationServer};
impl<H: Application> ApplicationServer<H> {
    /// Run the MCP server until transport closure.
    pub async fn run(&self) -> anyhow::Result<()> {
        crate::run(self).await
    }
}
