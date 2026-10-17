//! Bind one stream request to the shared execution driver.
use super::*;
pub(super) async fn run(
    tasks: Arc<DashMap<String, Task>>,
    context_id: Option<String>,
    spawn_task_id: String,
    bus: Option<Arc<crate::bus::AgentBus>>,
    prompt: String,
) {
    let mut host = host::Host {
        tasks: &tasks,
        bus: &bus,
        task_id: &spawn_task_id,
    };
    codetether_a2a_http::task_state::stream::driver::run(
        &tasks,
        context_id,
        &spawn_task_id,
        &prompt,
        &mut host,
    )
    .await;
}
