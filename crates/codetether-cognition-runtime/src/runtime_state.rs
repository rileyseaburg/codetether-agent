//! Application facade over shared cognition runtime state.
use crate::Host;

use std::ops::{Deref, DerefMut};
/// In-memory cognition runtime for perpetual persona swarms.
///
/// # Examples
/// ```no_run
/// use codetether_cognition_runtime::{CognitionRuntime, Host};
/// fn create<H: Host>() -> CognitionRuntime<H> {
///     CognitionRuntime::new_with_options(Default::default())
/// }
/// ```
pub struct CognitionRuntime<H: Host> {
    pub(super) state: codetether_cognition_core::runtime::CognitionRuntime<H::Thinker, H::Registry>,
}
impl<H: Host> Deref for CognitionRuntime<H> {
    type Target = codetether_cognition_core::runtime::CognitionRuntime<H::Thinker, H::Registry>;
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
impl<H: Host> DerefMut for CognitionRuntime<H> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}
impl<H: Host> std::fmt::Debug for CognitionRuntime<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.state, f)
    }
}
