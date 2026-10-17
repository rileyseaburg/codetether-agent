//! Native registry binding for shared lifecycle publication.
#[path = "bus_publish/host.rs"]
mod host;
#[path = "lifecycle.rs"]
pub(super) mod lifecycle;
pub(super) fn announce_working(agent_id: &str, summary: impl Into<String>) {
    codetether_agent_status::publish::announce_working::<host::ApplicationHost>(agent_id, summary);
}
pub(super) fn announce_done(agent_id: &str, success: bool, summary: impl Into<String>) {
    codetether_agent_status::publish::announce_done::<host::ApplicationHost>(
        agent_id, success, summary,
    );
}
pub(super) fn announce_result(agent_id: &str, response: &str, error: Option<&str>) {
    codetether_agent_status::publish::announce_result::<host::ApplicationHost>(
        agent_id, response, error,
    );
}
