//! Forward accepted event payloads to task activity and the bus.
use super::*;

pub(super) async fn forward<Event>(
    mut event_rx: mpsc::Receiver<Event>,
    bus_clone: Option<Arc<AgentBus>>,
    activity_tasks: Arc<DashMap<String, Task>>,
    task_id_clone: String,
    event_data: fn(&Event) -> Option<serde_json::Value>,
) {
    while let Some(event) = event_rx.recv().await {
        let event_data = match event_data(&event) {
            Some(data) => data,
            None => continue,
        };
        activity::record(&activity_tasks, &task_id_clone, &event_data);
        if let Some(ref bus) = bus_clone {
            bus.handle("a2a-stream").send(
                format!("task.{}", task_id_clone),
                BusMessage::TaskUpdate {
                    task_id: task_id_clone.clone(),
                    state: TaskState::Working,
                    message: Some(serde_json::to_string(&event_data).unwrap_or_default()),
                },
            );
        }
    }
}
