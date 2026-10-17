//! Forward session events to task activity and the bus.
use super::*;
pub(super) async fn forward(
    mut event_rx: mpsc::Receiver<SessionEvent>,
    bus_clone: Option<Arc<crate::bus::AgentBus>>,
    activity_tasks: Arc<DashMap<String, Task>>,
    task_id_clone: String,
) {
    while let Some(event) = event_rx.recv().await {
        let event_data = match event_data::data(&event) {
            Some(data) => data,
            None => continue,
        };
        server_task_activity::record(&activity_tasks, &task_id_clone, &event_data);
        if let Some(ref bus) = bus_clone {
            bus.handle("a2a-stream").send(
                format!("task.{}", task_id_clone),
                crate::bus::BusMessage::TaskUpdate {
                    task_id: task_id_clone.clone(),
                    state: crate::a2a::types::TaskState::Working,
                    message: Some(serde_json::to_string(&event_data).unwrap_or_default()),
                },
            );
        }
    }
}

/// Start forwarding with owned copies of the current bus and task state.
pub(super) fn spawn(
    bus: &Option<Arc<crate::bus::AgentBus>>,
    tasks: &Arc<DashMap<String, Task>>,
    task_id: &String,
    event_rx: mpsc::Receiver<SessionEvent>,
) {
    let (bus_clone, activity_tasks) = (bus.clone(), tasks.clone());
    let task_id_clone = task_id.clone();
    tokio::spawn(forward(event_rx, bus_clone, activity_tasks, task_id_clone));
}
