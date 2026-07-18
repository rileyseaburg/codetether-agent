//! Lossless parent wake-ups for child results and steered user input.

#[path = "parent_activity/channel.rs"]
mod channel;
#[path = "parent_activity/registry.rs"]
mod registry;
#[path = "parent_activity/wait.rs"]
mod wait;

pub(crate) use channel::Activity;

pub(crate) fn mailbox(owner: &str) {
    registry::publish(owner, Activity::Mailbox);
}

pub(crate) fn steered(owner: &str) {
    registry::publish(owner, Activity::Steered);
}

pub(crate) async fn until(owner: &str, deadline: tokio::time::Instant) -> Option<Activity> {
    wait::until(registry::channel(owner), deadline).await
}

#[cfg(test)]
#[path = "parent_activity/tests.rs"]
mod tests;
