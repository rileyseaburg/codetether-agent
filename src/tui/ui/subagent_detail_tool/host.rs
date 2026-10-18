//! Keep tool-child reads on the native parent-scoped registry bridge.
use crate::tool::agent::bridge;
use codetether_agent_trace::LiveTraceSnapshot;
use codetether_agent_view::AgentSnapshot;
use codetether_provider_core::Message;
pub(super) struct Registry;
impl codetether_agent_dashboard::tool_detail::Host for Registry {
    fn find(&self, name: &str, parent: &str) -> Option<AgentSnapshot> {
        bridge::find_agent_tool_agent_for_parent(name, parent)
    }
    fn transcript(&self, name: &str, parent: &str) -> Option<Vec<Message>> {
        bridge::agent_tool_transcript_for_parent(name, parent)
    }
    fn trace(&self, name: &str, parent: &str) -> Option<LiveTraceSnapshot> {
        bridge::agent_tool_live_trace_for_parent(name, parent)
    }
}
