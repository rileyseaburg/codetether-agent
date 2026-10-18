//! Native saved-policy and persistent-constraint bindings.
use crate::session::Session;
use codetether_context_policy::derivation::DerivePolicy;
impl codetether_context_dispatch::Source for Session {
    fn saved_policy(&self) -> DerivePolicy {
        self.metadata.context_policy
    }
    fn constraint_prefix(&self) -> Vec<String> {
        crate::session::store::constraints::prefix(self)
    }
    fn message_offset(&self) -> usize {
        Session::message_offset(self)
    }
}
