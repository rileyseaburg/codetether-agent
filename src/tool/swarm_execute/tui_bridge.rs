//! Process-local bridge from direct swarm workers to the TUI monitor.

#[path = "tui_bridge/cancel.rs"]
mod cancel;
#[path = "tui_bridge/channel.rs"]
mod channel;
#[path = "tui_bridge/finish.rs"]
mod finish;
#[path = "tui_bridge/observer.rs"]
mod observer;
#[path = "tui_bridge/stats.rs"]
mod stats;
#[path = "tui_bridge/task.rs"]
mod task;

pub(super) use observer::Observer;
pub(super) use task::started;

pub(super) fn task_id(task: &super::task_input::TaskInput, index: usize) -> String {
    task::id(task, index)
}

pub(crate) fn drain(view: &mut crate::tui::swarm_view::SwarmViewState) -> bool {
    channel::drain(view) | view.drain_events()
}

#[cfg(test)]
#[path = "tui_bridge_tests.rs"]
mod tests;
