//! Native live-registry lookup for shared snapshot-based agent views.
use crate::{session::Session, tool::agent::store};
/// Native session snapshots used by the shared agent view.
pub type ApplicationHost = codetether_agent_view::application::ApplicationHost<RegistrySource>;
/// Registry source preserving native parent resolution and name lookup.
pub struct RegistrySource;
impl codetether_agent_view::application::Source for RegistrySource {
    type Session = Session;
    fn entries(parent: Option<&str>) -> Vec<store::AgentEntry> {
        store::entries_for_parent(parent)
    }
    fn get_for_parent(name: &str, parent: &str) -> Option<store::AgentEntry> {
        store::get_for_parent(name, Some(parent))
    }
    fn contains(name: &str) -> bool {
        store::contains_name(name, None)
    }
}
