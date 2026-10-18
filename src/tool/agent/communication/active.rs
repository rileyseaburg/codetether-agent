//! Application specialization of immediate child steering.
pub(crate) async fn steer(target: &str, owner: Option<&str>, message: &str) -> super::Route {
    codetether_agent_communication::steer::<super::host::ApplicationHost>(target, owner, message)
        .await
}
