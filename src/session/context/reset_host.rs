//! Native runtime cache access for reset-policy derivation.
use crate::{provider::Provider, session::Session};
use std::sync::Arc;
impl codetether_reset_derivation::Source for Session {
    fn subcall_provider(&self) -> Option<&Arc<dyn Provider>> {
        self.metadata.subcall_provider.as_ref()
    }
    fn subcall_model_name(&self) -> Option<&str> {
        self.metadata.subcall_model_name.as_deref()
    }
}
