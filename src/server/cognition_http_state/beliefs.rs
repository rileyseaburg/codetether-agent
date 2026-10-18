//! Native cognition beliefs bindings.
use crate::cognition::CognitionRuntime;
use codetether_cognition_core::beliefs::Belief;
impl codetether_cognition_api::http::Beliefs for CognitionRuntime {
    async fn get_beliefs(&self) -> std::collections::HashMap<String, Belief> {
        CognitionRuntime::get_beliefs(self).await
    }
    async fn get_belief(&self, id: &str) -> Option<Belief> {
        CognitionRuntime::get_belief(self, id).await
    }
}
