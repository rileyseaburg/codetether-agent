//! Application local-name filtering for shared discovered-peer turns.
pub(in crate::tool::agent) use codetether_peer_agent::{message_or_missing, reply, result};
#[path = "observation.rs"]
pub(in crate::tool::agent) mod observation;
struct Names;
impl codetether_peer_agent::LocalNames for Names {
    fn contains(name: &str) -> bool {
        super::super::store::contains_name(name, None)
    }
}
pub(in crate::tool::agent) fn list() -> Vec<serde_json::Value> {
    codetether_peer_agent::list::<Names>()
}
