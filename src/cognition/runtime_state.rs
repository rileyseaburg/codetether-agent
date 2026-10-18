//! Application facade over shared cognition runtime state.
use super::ThinkerClient;
use crate::tool::ToolRegistry;
use std::ops::{Deref, DerefMut};
/// In-memory cognition runtime for perpetual persona swarms.
pub struct CognitionRuntime {
    pub(super) state:
        codetether_cognition_core::runtime::CognitionRuntime<ThinkerClient, ToolRegistry>,
}
impl Deref for CognitionRuntime {
    type Target = codetether_cognition_core::runtime::CognitionRuntime<ThinkerClient, ToolRegistry>;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl DerefMut for CognitionRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
impl std::fmt::Debug for CognitionRuntime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.state, f)
    }
}
