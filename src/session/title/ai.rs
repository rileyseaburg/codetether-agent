//! Native session specialization of caller-selected model title generation.
use crate::{provider::Provider, session::Session};
use anyhow::Result;
impl Session {
    pub(crate) async fn generate_ai_title_with(
        &mut self,
        provider: &dyn Provider,
        model: &str,
    ) -> Result<()> {
        codetether_session_title::generate_with(self, provider, model).await
    }
}
