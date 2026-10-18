//! Native context derivation and recall for shared request construction.
use crate::provider::Message;
use crate::session::Session;
use anyhow::Result;
use async_trait::async_trait;
#[async_trait]
impl codetether_context_request::Host for Session {
    async fn derive(
        &self,
        input: codetether_context_request::Derivation<'_>,
    ) -> Result<super::super::helpers::DerivedContext> {
        super::super::policy::derive_with_policy(
            self,
            input.provider,
            input.model,
            input.system_prompt,
            input.tools,
            None,
            input.policy,
            input.force_keep_last,
        )
        .await
    }
    async fn prefetch(&self) -> Option<Message> {
        crate::session::index::recall::prefetch::message(self).await
    }
}
