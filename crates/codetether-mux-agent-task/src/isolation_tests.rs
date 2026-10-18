//! Agent tasks are owned by one mux session and invisible to the others.

use std::sync::Arc;

use super::{entry::AgentTask, registry::AgentTaskRegistry, store::TaskStore};

fn registry_with(tasks: &[(&str, &str)]) -> AgentTaskRegistry {
    let registry = AgentTaskRegistry::new();
    let mut store = registry.store.lock().unwrap();
    for (id, session) in tasks {
        store.insert((*id).into(), Arc::new(AgentTask::new(0, session)));
    }
    drop(store);
    registry
}

#[tokio::test]
async fn reads_are_scoped_to_the_owning_session() {
    let registry = registry_with(&[("task-stripe", "stripe"), ("task-twilio", "twilio")]);
    let owner = registry.read("stripe", "task-stripe", 0).await;
    assert!(owner.is_ok());
    let stranger = registry.read("twilio", "task-stripe", 0).await;
    assert!(stranger.is_err(), "twilio must not observe stripe's task");
    assert!(registry.cancel("twilio", "task-stripe").is_err());
}

#[test]
fn one_running_turn_is_enforced_per_session_not_per_server() {
    let mut store = TaskStore::new();
    store.insert("task-stripe".into(), Arc::new(AgentTask::new(0, "stripe")));
    assert!(store.prepare("task-stripe-2", "stripe").is_err());
    assert!(store.prepare("task-twilio", "twilio").is_ok());
}
