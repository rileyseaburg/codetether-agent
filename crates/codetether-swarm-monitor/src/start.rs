//! Initial monitor events and observation timestamp.
use crate::{Observer, channel, task};
use codetether_swarm_tool::task_input::TaskInput;
use codetether_swarm_view::SwarmEvent;
use std::time::Instant;
impl Observer {
    /// Publish startup and task records before recording the observation start time.
    pub fn begin(tasks: &[TaskInput], max_steps: usize) -> Self {
        let events = channel::sender();
        let task = tasks
            .iter()
            .map(|task| task.name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let _ = events.try_send(SwarmEvent::Started {
            task: format!("Direct swarm: {task}"),
            total_subtasks: tasks.len(),
        });
        let subtasks = tasks
            .iter()
            .enumerate()
            .map(|(index, item)| task::info(item, index, max_steps))
            .collect();
        let _ = events.try_send(SwarmEvent::Decomposed { subtasks });
        Self {
            events,
            started_at: Instant::now(),
            terminal: false,
        }
    }
}
