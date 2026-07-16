//! Lifecycle owner for one direct `swarm_execute` invocation.

use super::super::{task_input::TaskInput, task_result::TaskResult};
use super::{channel, finish, task};
use crate::tui::swarm_view::SwarmEvent;
use std::time::Instant;
use tokio::sync::mpsc;

pub(in crate::tool::swarm_execute) struct Observer {
    pub(super) events: mpsc::Sender<SwarmEvent>,
    started_at: Instant,
    pub(super) terminal: bool,
}

impl Observer {
    pub(in crate::tool::swarm_execute) fn begin(tasks: &[TaskInput], max_steps: usize) -> Self {
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

    pub(in crate::tool::swarm_execute) fn sender(&self) -> mpsc::Sender<SwarmEvent> {
        self.events.clone()
    }

    pub(in crate::tool::swarm_execute) fn complete(&mut self, results: &[TaskResult]) {
        finish::send(&self.events, results, self.started_at.elapsed());
        self.terminal = true;
    }

    pub(in crate::tool::swarm_execute) fn fail(&mut self, error: impl Into<String>) {
        let _ = self.events.try_send(SwarmEvent::Error(error.into()));
        self.terminal = true;
    }
}
