//! Native cognition beliefs bindings.
use crate::{CognitionRuntime, Host};
use codetether_cognition_core::beliefs::Belief;
impl<H: Host> codetether_cognition_api::http::Beliefs for CognitionRuntime<H>
where
    H::Thinker: Send + Sync,
    H::Registry: Send + Sync,
{
    async fn get_beliefs(&self) -> std::collections::HashMap<String, Belief> {
        CognitionRuntime::get_beliefs(self).await
    }
    async fn get_belief(&self, id: &str) -> Option<Belief> {
        CognitionRuntime::get_belief(self, id).await
    }
}
